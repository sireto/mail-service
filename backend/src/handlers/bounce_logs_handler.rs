// https://f2d0-2400-74e0-0-6aae-be17-ceca-c77e-bb91.ngrok-free.app

use std::sync::Arc;

use crate::{models::{bounce_logs::
    { 
        BounceNotification, CreateBounceLogRequest, CreateBounceLogResponse, GetBounceLogResponse, SnsNotification
    }, mail::UpdateMailRequest}, repositories::{contact::ContactRepositoryImpl, mail_repository::MailRepositoryImpl}, services::{bounce_logs_service, contact::ContactService, mail_service::{self, MailService}}
};

use axum::{
    extract:: Path, Json, http::StatusCode
};
use uuid::Uuid;

enum MailStatus {
    Draft,
    Pending,
    Sent,
    Bounced,
}

impl MailStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MailStatus::Draft => "draft",
            MailStatus::Pending => "pending",
            MailStatus::Sent => "sent",
            MailStatus::Bounced => "bounced",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(MailStatus::Draft),
            "pending" => Some(MailStatus::Pending),
            "sent" => Some(MailStatus::Sent),
            "bounced" => Some(MailStatus::Bounced),
            _ => None,
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/bounce-logs/sns/bounce",
    responses(
        (status = 200, description = "Get the bounce notification from SNS and add it to the bounce log", body = CreateBounceLogResponse),
        (status = 404)
    )
)]
pub async fn handle_sns_notification (
    payload: Json<SnsNotification>,
) -> Result<(), (StatusCode, String)> {
     println!("THe payload ====> {payload:?}");

    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

     // automate the subscription confirmation...
     if payload.notification_type == "SubscriptionConfirmation" {
        // subscribe to the public api endpoint...
        let subscribe_url = serde_json::from_str::<serde_json::Value>(&payload.message)
            .ok()
            .and_then(|v| v["SubscribeURL"].as_str().map(String::from));

        if let Some(url) = subscribe_url {
            println!("Confirming SNS subscription...");
            reqwest::get(&url).await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        }
        return Ok(());
    }

    if payload.notification_type == "Notification" {
        let sns_event: BounceNotification = serde_json::from_str(&payload.message)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        match sns_event.notification_type.as_str() {
            "Bounce" => {
                if let Some(bounce) = sns_event.bounce {
                    let recipients = bounce.bounced_recipients;

                    let status = MailStatus::Bounced;
                    let mail_id = sns_event.mail.mail_id.clone();
                    let _ = mail_service.update_mail_status(mail_id, status.as_str()).await;

                    for recp in recipients {
                        let contact_repository = Arc::new(ContactRepositoryImpl);
                        let contact_service = ContactService::new(contact_repository);
                        let contact = contact_service.get_contact_by_email(recp.email_address).await;

                        let new_bounce = CreateBounceLogRequest {
                            contact_id: contact.unwrap().id,
                            at: bounce.timestamp.parse::<chrono::DateTime<chrono::Utc>>()
                                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
                            kind: bounce.bounce_type.clone(),
                            campaign_id: None,
                            reason: bounce.bounce_sub_type.clone(),
                            mail_id: sns_event.mail.mail_id.clone(),
                        };

                        // Add the bounce to the DB
                        bounce_logs_service::add_bounce(new_bounce).await?;
                    }
                }
            }
            "Delivery" => {
                if let Some(delivery) = sns_event.delivery {
                    let status = MailStatus::Sent;

                    let _ = mail_service.update_mail_status(sns_event.mail.mail_id, status.as_str()).await;
                }
            }
            _ => {
                println!("Unknown notification type: {}", sns_event.notification_type);
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
pub async fn get_all_bounces() -> Result<Json<Vec<GetBounceLogResponse>>, (StatusCode, String)> {
    let all_bounces_response = bounce_logs_service::get_all_bounces().await?;

    let response: Vec<GetBounceLogResponse> = all_bounces_response.into_iter().map(|bounce| GetBounceLogResponse {
        id: bounce.id,
        contact_id: bounce.contact_id,
        campaign_id: bounce.campaign_id,
        at: bounce.at,
        kind: bounce.kind,
        reason: bounce.reason,
    }).collect();

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
    Path(contact_id): Path<String>
) -> Result<Json<Vec<GetBounceLogResponse>>, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&contact_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let bounces_response = bounce_logs_service::get_bounces_by_contact_id(uuid_id).await?;

    let response: Vec<GetBounceLogResponse> = bounces_response.into_iter().map(|bounce| GetBounceLogResponse {
        id: bounce.id,
        contact_id: bounce.contact_id,
        campaign_id: bounce.campaign_id,
        at: bounce.at,
        kind: bounce.kind,
        reason: bounce.reason,
    }).collect();

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
pub async fn delete_bounce(
    Path(bounce_id): Path<String>
) -> Result<(), (StatusCode, String)> {
    // Convert the bounce_id of type string to the Uuid...
    let uuid_id = Uuid::parse_str(&bounce_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;
    
    bounce_logs_service::delete_bounce(uuid_id).await?;

    Ok(())
}