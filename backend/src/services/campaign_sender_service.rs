use crate::{error::AppError, repositories::campaign_sender::{CampaignSenderRepository, CampaignSenderRepositoryImpl}};
use multipart::server::nickel::nickel::hyper::method::Method::Delete;
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

pub async fn create_campaign_sender(payload: CreateCampaignSenderRequest) -> Result<CreateCampaignSenderResponse, AppError> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);
    let response = sender_service.create_campaign_sender(payload).await?;

    Ok(CreateCampaignSenderResponse {
        id: response.id,
        server_id: response.server_id,
        from_name: response.from_name,
        from_email: response.from_email,
    })
}

pub async fn get_all_campaign_senders() -> Result<Vec<GetCampaignSenderResponse>, AppError> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);
    let all_senders = sender_service.get_all_campaign_senders().await?;

    let senders = all_senders.into_iter().map(|sender| GetCampaignSenderResponse {
        id: sender.id,
        server_id: sender.server_id,
        from_name: sender.from_name,
        from_email: sender.from_email,
        created_at: sender.created_at,
        updated_at: sender.updated_at,
    }).collect();

    Ok(senders)
}

pub async fn get_campaign_sender_by_id(sender_id: String) -> Result<GetCampaignSenderResponse, AppError> {
    let uuid_id = Uuid::parse_str(&sender_id)?;

    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);
    let sender = sender_service.get_campaign_sender_by_id(uuid_id).await;

    let sender = sender.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

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
    sender_id: Uuid,
    payload: UpdateCampaignSenderRequest
) -> Result<UpdateCampaignSenderResponse, AppError> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let updated_sender_response = sender_service.update_campaign_sender(sender_id, payload).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(UpdateCampaignSenderResponse {
        id: updated_sender_response.id,
        server_id: updated_sender_response.server_id,
        from_name: updated_sender_response.from_name,
        from_email: updated_sender_response.from_email,
        updated_at: updated_sender_response.updated_at,
    })
}

pub async fn delete_campaign_sender(
    sender_id: Uuid,
) -> Result<DeleteCampaignSenderResponse, AppError> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let deleted_sender_response = sender_service.delete_campaign_sender(sender_id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(DeleteCampaignSenderResponse {
        id: deleted_sender_response.id,
        server_id: deleted_sender_response.server_id,
        from_name: deleted_sender_response.from_name,
        from_email: deleted_sender_response.from_email,
    })
}