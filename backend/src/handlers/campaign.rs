use uuid::Uuid;
use crate::error::AppError;
use crate::{models::{campaign::
    {
    CampaignSendResponse, CreateCampaignRequest, CreateCampaignResponse, DeleteCampaignResponse, ExtendedCreateCampaignRequest, GetCampaignResponse, UpdateCampaignRequest, UpdateCampaignResponse 
    }, contact::{DeleteContactResponse, UpdateContactResponse}}, repositories::campaign_sender::CampaignSenderRepositoryImpl, services::{campaign_sender_service, campaign_service} 
};


use axum::{
    extract:: Path, Json, http::StatusCode
};
use uuid::Uuid;

#[utoipa::path(
    post, 
    path="/api/campaigns", 
    responses(
        (status=200, description="Create a campaign", body= CreateCampaignRequest),
        (status=404)
    )
)]
pub async fn create_campaign(
    Json(payload): Json<ExtendedCreateCampaignRequest>
)->Result<Json<CreateCampaignResponse>, AppError>{

    let created_campaign = campaign_service::create_campaign(payload).await?;
    Ok(Json(created_campaign))
}

#[utoipa::path(
    get,
    path = "/api/campaigns",
    responses(
        (status = 200, description = "Get all the campaigns", body = Vec<GetCampaignResponse>),
        (status = 404)
    )
)]
pub async fn get_all_campaigns() -> Result<Json<Vec<GetCampaignResponse>>, AppError> {
    let campaigns = campaign_service::get_all_campaigns().await?;

    if campaigns.is_empty(){
        return Ok(Json(vec![]));
    }
    Ok(Json(campaigns))
}

#[utoipa::path(
    get,
    path = "/api/campaigns/{campaign_id}",
    responses(
        (status = 200, description = "Get a campaign by id", body = GetCampaignResponse),
        (status = 404)
    )
)]
pub async fn get_campaign_by_id(Path(campaign_id): Path<String>) -> Result<Json<GetCampaignResponse>, AppError> {
    let uuid_id = Uuid::parse_str(&campaign_id)?;

    let campaign = campaign_service::get_campaign_by_id(uuid_id).await?;

    Ok(Json(campaign))
}

#[utoipa::path(
    patch,
    path = "/api/campaigns/{campaign_id}",
    params(
        ("campaign_id" = String, Path, description = "ID of the campaign to update")
    ),
    responses(
        (status = 200, description = "Campaign updated successfully", body = UpdateContactResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Campaign not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_campaign(

    Path(campaign_id): Path<String>,
    Json(payload): Json<UpdateCampaignRequest>
) -> Result<Json<UpdateCampaignResponse>, AppError> {
    let uuid_id = Uuid::parse_str(&campaign_id)?;

    let update_contact_response = campaign_service::update_campaign(uuid_id, payload).await?;

    Ok(Json(update_contact_response))
}

#[utoipa::path(
    delete,
    path = "/api/campaigns/{campaign_id}",
    params(
        ("campaign_id" = String, Path, description = "ID of the campaign to delete") // ✅ Fixed parameter name
    ),
    responses(
        (status = 200, description = "Campaign deleted successfully", body = DeleteCampaignResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Campaign not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_campaign(
    Path(campaign_id): Path<String>
) -> Result<Json<DeleteCampaignResponse>, AppError> {
    let uuid_id = Uuid::parse_str(&campaign_id)?;
    let delete_campaign_response = campaign_service::delete_campaign(uuid_id).await?;

    Ok(Json(delete_campaign_response))
}


#[utoipa::path(
    post,
    path = "/api/campaigns/{campaign_id}/send",
    params(
        ("campaign_id" = String, Path, description = "ID of the campaign to send")
    ),
    responses(
        (status = 200, description = "Send campaign email", body = CampaignSendResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Campaign not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn send_campaign_email(
    Path(campaign_id): Path<String>
) -> Result<Json<CampaignSendResponse>, AppError> {
    let uuid_id = Uuid::parse_str(&campaign_id)?;
    let result = campaign_service::send_campaign_email(uuid_id).await?;

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/api/campaigns/{campaign_id}/send/smtp",
    params(
        ("campaign_id" = String, Path, description = "ID of the campaign to send"),
    ),
    responses(
        (status = 200, description = "Send campaign email via SMTP", body = CampaignSendResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Campaign not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn send_campaign_email_smtp(
    Path(campaign_id): Path<String>,
) -> Result<Json<CampaignSendResponse>, (StatusCode, String)> {
    use crate::services::campaign_service::send_campaign_email_smtp;
    
    let server_id = "ee1cc08b-fb4d-44e6-8a48-3a42e1b250a4";
    let campaign_sender_id = campaign_service::get_campaign_by_id(campaign_id.clone()).await.unwrap().campaign_senders.unwrap().to_string();
    let campaign_sender = campaign_sender_service::get_campaign_sender_by_id(campaign_sender_id).await?;

    let server_uuid = Uuid::parse_str(server_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid server_id".to_string()))?;
    let server_uuid = campaign_sender.server_id;

    let result = send_campaign_email_smtp(campaign_id, server_uuid).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}