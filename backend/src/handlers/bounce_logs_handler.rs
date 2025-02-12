// https://f2d0-2400-74e0-0-6aae-be17-ceca-c77e-bb91.ngrok-free.app

use std::sync::Arc;

use crate::{models::bounce_logs::
    { 
        BounceNotification, CreateBounceLogRequest, CreateBounceLogResponse, GetBounceLogResponse, SnsNotification
    }, repositories::contact::ContactRepositoryImpl, services::{bounce_logs_service, contact::ContactService}
};

use axum::{
    extract:: Path, Json, http::StatusCode
};
use uuid::Uuid;

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
        let bounced_notification: BounceNotification = serde_json::from_str(&payload.message)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        if bounced_notification.notification_type == "Bounce" {
            let receipent = bounced_notification.clone().bounce.bounced_recipients;

                for recp in receipent {
                    println!("The bounced email is: {recp:?}");
                    println!("THe bounced notification is::::::> {bounced_notification:?}");

                    let contact_repository = Arc::new(ContactRepositoryImpl);
                    let contact_service = ContactService::new(contact_repository);
                    let contact = contact_service.get_contact_by_email(recp.email_address).await;

                    let new_bounce = CreateBounceLogRequest {
                    contact_id: contact.unwrap().id,
                    at: bounced_notification.bounce.timestamp.parse::<chrono::DateTime<chrono::Utc>>().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
                    kind: bounced_notification.bounce.bounce_type.clone(),
                    campaign_id: None,
                    reason: bounced_notification.bounce.bounce_sub_type.clone(),
                };

                // add the bounce to the db after the parsed notification is of type bounce...
                bounce_logs_service::add_bounce(new_bounce).await?;
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