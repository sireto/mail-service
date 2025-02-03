use crate::{models::{campaign::
    {
    CampaignSendResponse, CreateCampaignRequest, CreateCampaignResponse, DeleteCampaignResponse, GetCampaignResponse, UpdateCampaignRequest, UpdateCampaignResponse 
    }, contact::{DeleteContactResponse, UpdateContactResponse}}, services::campaign, 
};


use axum::{
    extract:: Path, Json, http::StatusCode
};

#[utoipa::path(
    post, 
    path="/api/campaigns", 
    responses(
        (status=200, description="Create a campaign", body= CreateCampaignRequest),
        (status=404)
    )
)]
pub async fn create_campaign(
    Json(payload): Json<CreateCampaignRequest>
)->Result<Json<CreateCampaignResponse>, (StatusCode, String)>{

    let created_campaign = campaign::create_campaign(payload).await?;
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
pub async fn get_all_campaigns() -> Result<Json<Vec<GetCampaignResponse>>, (StatusCode, String)> {
    let campaigns = campaign::get_all_campaigns().await?;

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
pub async fn get_campaign_by_id(Path(campaign_id): Path<String>) -> Result<Json<GetCampaignResponse>, (StatusCode, String)> {
    let campaign = campaign::get_campaign_by_id(campaign_id).await?;

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
    
    Path(contact_id): Path<String>,
    Json(payload): Json<UpdateCampaignRequest>
) -> Result<Json<UpdateCampaignResponse>, (StatusCode, String)> {

    let update_contact_response = campaign::update_campaign(contact_id, payload).await?;

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
) -> Result<Json<DeleteCampaignResponse>, (StatusCode, String)> {
    let delete_campaign_response = campaign::delete_campaign(campaign_id).await?;

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
) -> Result<Json<CampaignSendResponse>, (StatusCode, String)> {
    
    let result = campaign::send_campaign_email(campaign_id).await;

    match result {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}