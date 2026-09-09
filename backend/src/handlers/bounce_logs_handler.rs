use std::sync::Arc;

use crate::{
    error::AppError,
    models::{
        bounce_logs::{
            CreateBounceLogRequest, CreateBounceLogResponse, GetBounceLogResponse, Message, SnsNotification,
        },
        mail::UpdateMailRequest,
    },
    services::{
        bounce_logs_service,
        mail_service::{MailService, MailServiceTrait, MAIL_STATUS_BOUNCED, MAIL_STATUS_DELIVERED},
    },
};

use crate::utils::bounce_logs::{is_trusted_sns_url, search_header_from_header_list};
use axum::{extract::Path, Extension, Json};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/bounce-logs/sns/bounce",
    responses(
        (status = 200, description = "Get the bounce notification from SNS and add it to the bounce log", body = CreateBounceLogResponse),
        (status = 404)
    )
)]
pub async fn handle_sns_notification(
    Extension(mail_service): Extension<Arc<MailService>>,
    payload: Json<SnsNotification>,
) -> Result<(), AppError> {
    // Restrict the endpoint to a known topic when one is configured. This is a partial
    // control: it is not a substitute for verifying the SNS message signature, which is
    // still outstanding.
    if let Ok(expected_topic) = std::env::var("AWS_SNS_TOPIC_ARN") {
        let expected_topic = expected_topic.trim();
        if !expected_topic.is_empty() && payload.topic_arn.as_deref() != Some(expected_topic) {
            eprintln!("Rejecting SNS notification for unexpected topic");
            return Err(AppError::BadRequestError(Some(
                "Notification topic is not accepted by this endpoint".to_string(),
            )));
        }
    }

    // automate the subscription confirmation...
    if payload.notification_type == "SubscriptionConfirmation" {
        // subscribe to the public api endpoint...
        let subscribe_url = serde_json::from_str::<serde_json::Value>(&payload.message)
            .ok()
            .and_then(|v| v["SubscribeURL"].as_str().map(String::from));

        if let Some(url) = subscribe_url {
            // Never fetch a URL just because the request body said so.
            if !is_trusted_sns_url(&url) {
                eprintln!("Refusing SNS subscription confirmation for untrusted URL");
                return Err(AppError::BadRequestError(Some(
                    "SubscribeURL is not an Amazon SNS endpoint".to_string(),
                )));
            }

            println!("Confirming SNS subscription...");
            reqwest::get(&url)
                .await
                .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;
        }
        return Ok(());
    }

    if payload.notification_type == "Notification" {
        let sns_event: Message =
            serde_json::from_str(&payload.message).map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

        if sns_event.notification_type.is_none() {
            if let Some(event_type) = sns_event.event_type.as_ref() {
                let mail_id = search_header_from_header_list(&sns_event, "mailId")
                    .ok_or_else(|| AppError::NotFoundError(Some("mailId not found in SNS event".to_string())))?;

                match event_type.as_str() {
                    "Open" => {
                        mail_service
                            .update_mail(
                                mail_id,
                                UpdateMailRequest {
                                    open: Some(chrono::Utc::now()),
                                    ..Default::default()
                                },
                            )
                            .await?;
                    }
                    "Click" => {
                        mail_service.increment_mail_clicks(mail_id).await?;
                    }
                    _ => {
                        println!("Unknown event type: {}", event_type);
                    }
                }
            } else {
                println!("SNS event carried neither notificationType nor eventType; ignoring");
            }

            return Ok(());
        }

        let Some(notification_type) = sns_event.notification_type.as_deref() else {
            return Ok(());
        };

        match notification_type {
            "Bounce" => {
                if let Some(bounce) = sns_event.bounce.as_ref() {
                    let recipients = &bounce.bounced_recipients;

                    let mail_id = search_header_from_header_list(&sns_event, "mailId")
                        .ok_or_else(|| AppError::NotFoundError(Some("mailId not found in SNS event".to_string())))?;

                    let _ = mail_service
                        .update_mail_status(mail_id.clone(), MAIL_STATUS_BOUNCED)
                        .await;

                    // The mail row is the source of truth for both the campaign and the
                    // contact. campaign_id was hardcoded to None, so the bounces table's
                    // campaign column was always blank; and resolving the contact from the
                    // bounced address alone is now ambiguous, since an email is only unique
                    // within a namespace and this webhook carries no namespace of its own.
                    let bounced_mail = mail_service.get_mail_by_id(mail_id.clone()).await.ok();
                    let campaign_id = bounced_mail.as_ref().and_then(|mail| mail.campaign_id);
                    let bounced_contact_id = bounced_mail.as_ref().map(|mail| mail.contact_id);

                    for recp in recipients {
                        let Some(contact_id) = bounced_contact_id else {
                            // A bounce can name a mail we never stored. That is not a reason
                            // to abort the request.
                            eprintln!(
                                "Bounce names unknown mail {mail_id}; skipping bounce log for {}",
                                recp.email_address
                            );
                            continue;
                        };

                        let new_bounce = CreateBounceLogRequest {
                            contact_id,
                            at: bounce
                                .timestamp
                                .parse::<chrono::DateTime<chrono::Utc>>()
                                .map_err(|err| AppError::BadRequestError(Some(err.to_string())))?,
                            kind: bounce.bounce_type.clone(),
                            campaign_id,
                            reason: bounce.bounce_sub_type.clone(),
                            mail_id: mail_id.clone(),
                        };

                        // Add the bounce to the DB
                        bounce_logs_service::add_bounce(new_bounce).await?;
                    }
                }
            }
            "Delivery" => {
                if sns_event.delivery.is_some() {
                    let mail_id = search_header_from_header_list(&sns_event, "mailId")
                        .ok_or_else(|| AppError::NotFoundError(Some("mailId not found in SNS event".to_string())))?;

                    let _ = mail_service.update_mail_status(mail_id, MAIL_STATUS_DELIVERED).await;
                }
            }
            other => {
                println!("Unknown notification type: {other}");
            }
        }
    }

    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/bounce-logs",
    responses(
        (status = 200, description = "Get the list of bounce logs", body = Vec<GetBounceLogResponse>),
        (status = 500)
    )
)]
pub async fn get_all_bounces() -> Result<Json<Vec<GetBounceLogResponse>>, AppError> {
    let all_bounces_response = bounce_logs_service::get_all_bounces().await?;

    let response: Vec<GetBounceLogResponse> = all_bounces_response
        .into_iter()
        .map(|bounce| GetBounceLogResponse {
            id: bounce.id,
            contact_id: bounce.contact_id,
            campaign_id: bounce.campaign_id,
            at: bounce.at,
            kind: bounce.kind,
            reason: bounce.reason,
        })
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/bounce-logs/contacts/{contact_id}",
    responses(
        (status = 200, description = "Get the list of bounce logs of the contact id", body = Vec<GetBounceLogResponse>),
        (status = 500)
    )
)]
pub async fn get_bounces_by_contact_id(
    Path(contact_id): Path<String>,
) -> Result<Json<Vec<GetBounceLogResponse>>, AppError> {
    let uuid_id = Uuid::parse_str(&contact_id)?;

    let bounces_response = bounce_logs_service::get_bounces_by_contact_id(uuid_id).await?;

    let response: Vec<GetBounceLogResponse> = bounces_response
        .into_iter()
        .map(|bounce| GetBounceLogResponse {
            id: bounce.id,
            contact_id: bounce.contact_id,
            campaign_id: bounce.campaign_id,
            at: bounce.at,
            kind: bounce.kind,
            reason: bounce.reason,
        })
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/bounce-logs/{bounce_id}",
    responses(
        (status = 200, description = "Delete a bounce from the log", body = ()),
        (status = 404)
    )
)]
pub async fn delete_bounce(Path(bounce_id): Path<String>) -> Result<(), AppError> {
    // Convert the bounce_id of type string to the Uuid...
    let uuid_id = Uuid::parse_str(&bounce_id)?;

    bounce_logs_service::delete_bounce(uuid_id).await?;

    Ok(())
}
