use crate::{models::campaign::{CampaignSendResponse, DeleteCampaignResponse, GetCampaignResponse, UpdateCampaignRequest, UpdateCampaignResponse}, repositories::{campaign::{self, CampaginRepositoryImpl, CampaignRepository}}};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::campaign::{
    Campaign, 
    CreateCampaignRequest, 
    CreateCampaignResponse,
};


pub struct CampaignService {
    repository: Arc<dyn CampaignRepository + Send +Sync>
}

impl CampaignService {
    pub fn new(repository: Arc<dyn CampaignRepository + Send + Sync>) -> Self {
        Self {repository}
    }
    pub async fn create_campaign(&self, payload: CreateCampaignRequest) -> Result<Campaign, diesel::result::Error> {
        self.repository.create_campaign(payload).await
    }
    pub async fn get_all_campaigns(&self) -> Result<Vec<Campaign>, diesel::result::Error> {
        self.repository.get_all_campaigns().await
    }
    pub async fn get_campaign_by_id(&self, campaign_id: Uuid)-> Result<Campaign, diesel::result::Error> {
        self.repository.get_campaign_by_id(campaign_id).await
    }
    pub async fn update_campaign(&self, campaign_id: Uuid, payload: UpdateCampaignRequest) -> Result<Campaign, diesel::result::Error> {
        self.repository.update_campaign(campaign_id, payload).await
    }
    pub async fn delete_campaign(&self, campaign_id: Uuid) -> Result<Campaign, diesel::result::Error>{
        self.repository.delete_campaign(campaign_id).await
    }
}



pub async fn create_campaign( payload: CreateCampaignRequest) -> Result<CreateCampaignResponse, (StatusCode, String)> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let response = campaign_service.create_campaign(payload).await;

    let response = match response {
        Ok(campaign) => CreateCampaignResponse {
            id: campaign.id, 
            campaign_name: campaign.campaign_name, 
            template_id: campaign.template_id, 
            namespace_id: campaign.namespace_id, 
            status: campaign.status, 
            campaign_senders: campaign.campaign_senders, 
            scheduled_at: campaign.scheduled_at, 
            created_at: campaign.created_at, 
            updated_at: campaign.updated_at
        }, 
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

pub async fn get_all_campaigns() -> Result<Vec<GetCampaignResponse>, (StatusCode, String)> {
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let all_campaigns = campaign_service.get_all_campaigns().await;

    let response = match all_campaigns {
        Ok(campaigns) => campaigns.into_iter().map(|campaign| GetCampaignResponse {
            id: campaign.id,
            campaign_name: campaign.campaign_name,
            template_id: campaign.template_id,
            namespace_id: campaign.namespace_id,
            status: campaign.status,
            campaign_senders: campaign.campaign_senders,
            scheduled_at: campaign.scheduled_at,
            created_at: campaign.created_at,
            updated_at: campaign.updated_at,
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string())),
    };

    Ok(response)
}

pub async fn get_campaign_by_id(campaign_id: String) -> Result<GetCampaignResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&campaign_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid campaign ID format".to_string()))?;

    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    let campaign = campaign_service.get_campaign_by_id(uuid_id).await;

    let campaign = campaign.map_err(|err|(StatusCode::NOT_FOUND, err.to_string()))?;

    let campaign_response = GetCampaignResponse{
        id: campaign.id, 
        campaign_name: campaign.campaign_name, 
        template_id: campaign.template_id, 
        namespace_id: campaign.namespace_id, 
        status: campaign.status, 
        campaign_senders: campaign.campaign_senders, 
        scheduled_at: campaign.scheduled_at, 
        created_at: campaign.created_at, 
        updated_at: campaign.updated_at,
    };

    Ok(campaign_response)
}

pub async fn update_campaign(campaign_id: String, payload: UpdateCampaignRequest) -> Result<UpdateCampaignResponse, (StatusCode, String)> {
    
    let uuid_id = Uuid::parse_str(&campaign_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid campaign ID format".to_string()))?;
    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);
    
    let updated_campaign_response = campaign_service.update_campaign(uuid_id, payload).await;

    let response_campaign = match updated_campaign_response {
        Ok(campaign) => UpdateCampaignResponse {
            id: campaign.id, 
            campaign_name: campaign.campaign_name, 
            template_id:  campaign.template_id, 
            namespace_id: campaign.namespace_id, 
            status: campaign.status, 
            campaign_senders: campaign.campaign_senders, 
            scheduled_at: Some(campaign.scheduled_at), 
            updated_at: Some(campaign.updated_at)
        }, 
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response_campaign)
}

pub async fn delete_campaign(campaign_id: String)->Result<DeleteCampaignResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&campaign_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let campaign_repository = Arc::new(CampaginRepositoryImpl);
    let campaign_service = CampaignService::new(campaign_repository);

    let deleted_campaign_response = campaign_service.delete_campaign(uuid_id).await;

    let deleted_campaign = match deleted_campaign_response {
        Ok(campaign) => DeleteCampaignResponse {
            id: campaign.id, 
            campaign_name: campaign.campaign_name, 
            status: campaign.status
        }, 
        Err(err)=> return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(deleted_campaign)
}
