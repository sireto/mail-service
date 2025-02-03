use crate::repositories::campaign_sender::{CampaignSenderRepository, CampaignSenderRepositoryImpl};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::campaign_sender::{
    CampaignSender,
    CreateCampaignSenderRequest,
    CreateCampaignSenderResponse,
    GetCampaignSenderResponse,
    UpdateCampaignSenderRequest,
    UpdateCampaignSenderResponse,
    DeleteCampaignSenderResponse
};

pub struct CampaignSenderService {
    repository: Arc<dyn CampaignSenderRepository + Send + Sync>
}

impl CampaignSenderService {
    pub fn new(repository: Arc<dyn CampaignSenderRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_campaign_sender(&self, payload: CreateCampaignSenderRequest) -> Result<CampaignSender, diesel::result::Error> {
        self.repository.create_campaign_sender(payload).await
    }

    pub async fn get_all_campaign_senders(&self) -> Result<Vec<CampaignSender>, diesel::result::Error> {
        self.repository.get_all_campaign_senders().await
    }

    pub async fn get_campaign_sender_by_id(&self, sender_id: Uuid) -> Result<CampaignSender, diesel::result::Error> {
        self.repository.get_campaign_sender_by_id(sender_id).await
    }

    pub async fn update_campaign_sender(
        &self,
        sender_id: Uuid,
        payload: UpdateCampaignSenderRequest
    ) -> Result<CampaignSender, diesel::result::Error> {
        self.repository.update_campaign_sender(sender_id, payload).await
    }

    pub async fn delete_campaign_sender(
        &self,
        sender_id: Uuid,
    ) -> Result<CampaignSender, diesel::result::Error> {
        self.repository.delete_campaign_sender(sender_id).await
    }
}

pub async fn create_campaign_sender(payload: CreateCampaignSenderRequest) -> Result<CreateCampaignSenderResponse, (StatusCode, String)> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);
    let response = sender_service.create_campaign_sender(payload).await;

    let response = match response {
        Ok(sender) => CreateCampaignSenderResponse {
            id: sender.id,
            server_id: sender.server_id,
            from_name: sender.from_name,
            from_email: sender.from_email,
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

pub async fn get_all_campaign_senders() -> Result<Vec<GetCampaignSenderResponse>, (StatusCode, String)> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);
    let all_senders = sender_service.get_all_campaign_senders().await;

    let response = match all_senders {
        Ok(senders) => senders.into_iter().map(|sender| GetCampaignSenderResponse {
            id: sender.id,
            server_id: sender.server_id,
            from_name: sender.from_name,
            from_email: sender.from_email,
            created_at: sender.created_at,
            updated_at: sender.updated_at,
        }).collect(),
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response)
}

pub async fn get_campaign_sender_by_id(sender_id: String) -> Result<GetCampaignSenderResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&sender_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid sender ID format".to_string()))?;

    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);
    let sender = sender_service.get_campaign_sender_by_id(uuid_id).await;

    let sender = sender.map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    let sender_response = GetCampaignSenderResponse {
        id: sender.id,
        server_id: sender.server_id,
        from_name: sender.from_name,
        from_email: sender.from_email,
        created_at: sender.created_at,
        updated_at: sender.updated_at,
    };

    Ok(sender_response)
}

pub async fn update_campaign_sender(
    sender_id: String,
    payload: UpdateCampaignSenderRequest
) -> Result<UpdateCampaignSenderResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&sender_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid sender ID format".to_string()))?;

    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let updated_sender_response = sender_service.update_campaign_sender(uuid_id, payload).await;

    let response_sender = match updated_sender_response {
        Ok(sender) => UpdateCampaignSenderResponse {
            id: sender.id,
            server_id: sender.server_id,
            from_name: sender.from_name,
            from_email: sender.from_email,
            updated_at: sender.updated_at,
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response_sender)
}

pub async fn delete_campaign_sender(
    sender_id: String,
) -> Result<DeleteCampaignSenderResponse, (StatusCode, String)> {
    let uuid_id = Uuid::parse_str(&sender_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let deleted_sender_response = sender_service.delete_campaign_sender(uuid_id).await;

    let response_sender = match deleted_sender_response {
        Ok(sender) => DeleteCampaignSenderResponse {
            id: sender.id,
            server_id: sender.server_id,
            from_name: sender.from_name,
            from_email: sender.from_email,
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err.to_string()))
    };

    Ok(response_sender)
}