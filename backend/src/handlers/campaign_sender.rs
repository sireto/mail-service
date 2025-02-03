use crate::{
    models::campaign_sender::{
        CampaignSender,
        CreateCampaignSenderRequest,
        CreateCampaignSenderResponse,
        GetCampaignSenderResponse,
        UpdateCampaignSenderRequest,
        UpdateCampaignSenderResponse,
        DeleteCampaignSenderResponse
    },
    services::campaign_sender_service
};
use axum::{
    extract::Path, Json, http::StatusCode
};
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "/api/campaign-senders",
    request_body = CreateCampaignSenderRequest,
    responses(
        (status = 201, description = "Create a campaign sender", body = CreateCampaignSenderResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_campaign_sender(
    Json(payload): Json<CreateCampaignSenderRequest>,
) -> Result<Json<CreateCampaignSenderResponse>, (StatusCode, String)> {
    let created_sender = campaign_sender_service::create_campaign_sender(payload).await?;
    Ok(Json(created_sender))
}

#[utoipa::path(
    get,
    path = "/api/campaign-senders",
    responses(
        (status = 200, description = "List all campaign senders", body = Vec<GetCampaignSenderResponse>),
        (status = 404, description = "No senders found")
    )
)]
pub async fn get_campaign_senders() -> Result<Json<Vec<GetCampaignSenderResponse>>, (StatusCode, String)> {
    let senders = campaign_sender_service::get_all_campaign_senders().await?;

    if senders.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No campaign senders found".to_string()));
    }

    Ok(Json(senders))
}

#[utoipa::path(
    get,
    path = "/api/campaign-senders/{sender_id}",
    params(
        ("sender_id" = String, Path, description = "Campaign sender ID")
    ),
    responses(
        (status = 200, description = "Get campaign sender by ID", body = GetCampaignSenderResponse),
        (status = 400, description = "Invalid ID format"),
        (status = 404, description = "Sender not found")
    )
)]
pub async fn get_campaign_sender_by_id(
    Path(sender_id): Path<String>
) -> Result<Json<GetCampaignSenderResponse>, (StatusCode, String)> {
    let sender = campaign_sender_service::get_campaign_sender_by_id(sender_id).await?;
    Ok(Json(sender))
}

#[utoipa::path(
    patch,
    path = "/api/campaign-senders/{sender_id}",
    params(
        ("sender_id" = String, Path, description = "Campaign sender ID")
    ),
    request_body = UpdateCampaignSenderRequest,
    responses(
        (status = 200, description = "Updated campaign sender", body = UpdateCampaignSenderResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Sender not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_campaign_sender(
    Path(sender_id): Path<String>,
    Json(payload): Json<UpdateCampaignSenderRequest>
) -> Result<Json<UpdateCampaignSenderResponse>, (StatusCode, String)> {
    let updated_sender = campaign_sender_service::update_campaign_sender(sender_id, payload).await?;
    Ok(Json(updated_sender))
}

#[utoipa::path(
    delete,
    path = "/api/campaign-senders/{sender_id}",
    params(
        ("sender_id" = String, Path, description = "Campaign sender ID")
    ),
    responses(
        (status = 200, description = "Deleted campaign sender", body = DeleteCampaignSenderResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Sender not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_campaign_sender(
    Path(sender_id): Path<String>
) -> Result<Json<DeleteCampaignSenderResponse>, (StatusCode, String)> {
    let deleted_sender = campaign_sender_service::delete_campaign_sender(sender_id).await?;
    Ok(Json(deleted_sender))
}