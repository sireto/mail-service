use crate::repositories::bounce_logs_repo::{ self, BounceLogsRepository, BounceLogsRepositoryImpl };
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::bounce_logs::{
    BounceLog,
    CreateBounceLogRequest,
    CreateBounceLogResponse
};

pub struct BounceLogsService {
    repository: Arc<dyn BounceLogsRepository + Send + Sync>
}

impl BounceLogsService {
    pub fn new(repository: Arc<dyn BounceLogsRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn add_bounce(&self, payload: CreateBounceLogRequest) -> Result<BounceLog, diesel::result::Error> {
        self.repository.add_bounce(payload).await
    }

    pub async fn get_all_bounces(&self) -> Result<Vec<BounceLog>, diesel::result::Error> {
        self.repository.get_all_bounces().await
    }

    pub async fn delete_bounce(&self, bounce_id: Uuid) -> Result<BounceLog, diesel::result::Error> {
        self.repository.delete_bounce(bounce_id).await
    }

    pub async fn get_bounces_by_contact_id(&self, bounce_contact_id: Uuid) -> Result<Vec<BounceLog>, diesel::result::Error> {
        self.repository.get_bounces_of_contact_id(bounce_contact_id).await
    }
}

/// a function to add a new bounce into the record...
pub async fn add_bounce(payload: CreateBounceLogRequest) -> Result<CreateBounceLogResponse, (StatusCode, String)> {
    let bounce_logs_repository = Arc::new(BounceLogsRepositoryImpl);
    let bounce_logs_service = BounceLogsService::new(bounce_logs_repository);
    let response = bounce_logs_service.add_bounce(payload).await;

    let response = match response {
        Ok(bounce_log) => CreateBounceLogResponse {
            id: bounce_log.id,
            contact: bounce_log.contact_id,
            campaign_id: bounce_log.campaign_id,
            at: bounce_log.at,
            kind: bounce_log.kind,
            reason: bounce_log.reason
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

/// a funtion to get the list of all the bounces...
pub async fn get_all_bounces() -> Result<Vec<BounceLog>, (StatusCode, String)> {
    let bounce_logs_repository = Arc::new(BounceLogsRepositoryImpl);
    let bounce_logs_service = BounceLogsService::new(bounce_logs_repository);
    let all_bounces = bounce_logs_service.get_all_bounces().await;

    let response = match all_bounces {
        Ok(bounces) => bounces.into_iter().map(|bounce| BounceLog {
            id: bounce.id,
            contact_id: bounce.contact_id,
            campaign_id: bounce.campaign_id,
            at: bounce.at,
            kind: bounce.kind,
            reason: bounce.reason,
        }).collect(),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

/// a function to get all contacts related bounce logs...
pub async fn get_bounces_by_contact_id(bounce_contact_id: Uuid) -> Result<Vec<BounceLog>, (StatusCode, String)> {
    let bounce_logs_repository = Arc::new(BounceLogsRepositoryImpl);
    let bounce_logs_service = BounceLogsService::new(bounce_logs_repository);
    let bounces = bounce_logs_service.get_bounces_by_contact_id(bounce_contact_id).await;

    let response = match bounces {
        Ok(bounces) => bounces,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)
}

/// a function to delete a bounce from the record...
pub async fn delete_bounce(bounce_id: Uuid) -> Result<BounceLog, (StatusCode, String)> {
    let bounce_logs_repository = Arc::new(BounceLogsRepositoryImpl);
    let bounce_logs_service = BounceLogsService::new(bounce_logs_repository);
    let deleted_bounce = bounce_logs_service.delete_bounce(bounce_id).await;

    let response = match deleted_bounce {
        Ok(bounce) => bounce,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    };

    Ok(response)

}