use crate::{models::campaign_sender::{CampaignSenderRequest, ValidateEmailIdentityRequest, ValidateEmailIdentityResponse}, repositories::campaign_sender::{CampaignSenderRepository, CampaignSenderRepositoryImpl}};
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
use chrono::Utc;
use super::aws_service;

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

pub async fn create_campaign_sender(payload: CampaignSenderRequest) -> Result<CreateCampaignSenderResponse, (StatusCode, String)> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let id = Uuid::parse_str(&payload.server_id).unwrap();

    let payload = CreateCampaignSenderRequest {
        server_id: id, 
        from_email: payload.from_email, 
        from_name: payload.from_name
    };

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

pub async fn validate_email_identity(payload: ValidateEmailIdentityRequest) -> Result<ValidateEmailIdentityResponse, (StatusCode, String)> {
    // Create AWS SES client
    let client = aws_service::create_aws_client().await;
    
    // Extract domain from email (for domain identity validation)
    let email = payload.email.clone();
    let domain = email.split('@').last().unwrap_or("").to_string();
    println!("{}", email);
    println!("{}", domain);
    
    // Get list of verified identities
    let request = client.list_email_identities();
    let result = request.send().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch identities: {}", e))
    })?;
    
    // Check if email or its domain is verified
    let mut is_valid = false;
    if let Some(identities) = result.email_identities {
        for identity in identities {
            if let Some(identity_name) = identity.identity_name.clone() {
                println!("{:?}", identity.identity_name.clone());
                // Check if the email is directly verified or if its domain is verified
                if identity_name == email || identity_name == domain {
                    is_valid = true;
                    break;
                }
            }
        }
    }
    
    if is_valid {
        println!("Email is validated");
        Ok(ValidateEmailIdentityResponse {
            is_valid: true,
            message: None,
        })
    } else {
        println!("Email is not validated");
        Ok(ValidateEmailIdentityResponse {
            is_valid: false,
            message: Some(format!("Email '{}' is not verified in AWS SES. Please verify it in the AWS console first.", email)),
        })
    }
}

// Function to get all verified identities
pub async fn get_verified_identities() -> Result<Vec<String>, (StatusCode, String)> {
    let client = aws_service::create_aws_client().await;
    let request = client.list_email_identities();
    
    let result = request.send().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch identities: {}", e))
    })?;
    
    let mut identities = Vec::new();
    if let Some(verified_identities) = result.email_identities {
        for identity in verified_identities {
            if let Some(name) = identity.identity_name {
                identities.push(name);
            }
        }
    }
    
    Ok(identities)
}