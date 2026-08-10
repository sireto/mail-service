use super::aws_service::{self, create_aws_client_db};
use crate::models::campaign_sender::{
    CampaignSender, CreateCampaignSenderRequest, CreateCampaignSenderResponse, DeleteCampaignSenderResponse,
    GetCampaignSenderResponse, UpdateCampaignSenderRequest, UpdateCampaignSenderResponse,
};
use crate::servers::servers_repo::ServerRepoImpl;
use crate::{
    error::AppError,
    models::campaign_sender::{
        CampaignSenderRequest, SendTestEmailRequest, SendTestEmailResponse, ValidateEmailIdentityRequest,
        ValidateEmailIdentityResponse,
    },
    repositories::campaign_sender::{CampaignSenderRepository, CampaignSenderRepositoryImpl},
    servers::{
        servers_handler::{get_server_by_id, get_servers},
        servers_model::{Server, ServerTypeEnum},
        servers_services::{self, ServerServiceTrait},
    },
};
use axum::http::StatusCode;
use chrono::Utc;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::sync::Arc;
use uuid::Uuid;

pub struct CampaignSenderService {
    repository: Arc<dyn CampaignSenderRepository + Send + Sync>,
}

impl CampaignSenderService {
    pub fn new(repository: Arc<dyn CampaignSenderRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_campaign_sender(
        &self,
        payload: CreateCampaignSenderRequest,
    ) -> Result<CampaignSender, diesel::result::Error> {
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
        payload: UpdateCampaignSenderRequest,
    ) -> Result<CampaignSender, diesel::result::Error> {
        self.repository.update_campaign_sender(sender_id, payload).await
    }

    pub async fn delete_campaign_sender(&self, sender_id: Uuid) -> Result<CampaignSender, diesel::result::Error> {
        self.repository.delete_campaign_sender(sender_id).await
    }
}

pub async fn create_campaign_sender(payload: CampaignSenderRequest) -> Result<CreateCampaignSenderResponse, AppError> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let id = Uuid::parse_str(&payload.server_id).unwrap();

    let payload = CreateCampaignSenderRequest {
        server_id: id,
        from_email: payload.from_email,
        from_name: payload.from_name,
    };

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

    let senders = all_senders
        .into_iter()
        .map(|sender| GetCampaignSenderResponse {
            id: sender.id,
            server_id: sender.server_id,
            from_name: sender.from_name,
            from_email: sender.from_email,
            created_at: sender.created_at,
            updated_at: sender.updated_at,
        })
        .collect();

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
    payload: UpdateCampaignSenderRequest,
) -> Result<UpdateCampaignSenderResponse, AppError> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let updated_sender_response = sender_service
        .update_campaign_sender(sender_id, payload)
        .await
        .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(UpdateCampaignSenderResponse {
        id: updated_sender_response.id,
        server_id: updated_sender_response.server_id,
        from_name: updated_sender_response.from_name,
        from_email: updated_sender_response.from_email,
        updated_at: updated_sender_response.updated_at,
    })
}

pub async fn delete_campaign_sender(sender_id: Uuid) -> Result<DeleteCampaignSenderResponse, AppError> {
    let sender_repository = Arc::new(CampaignSenderRepositoryImpl);
    let sender_service = CampaignSenderService::new(sender_repository);

    let deleted_sender_response = sender_service
        .delete_campaign_sender(sender_id)
        .await
        .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(DeleteCampaignSenderResponse {
        id: deleted_sender_response.id,
        server_id: deleted_sender_response.server_id,
        from_name: deleted_sender_response.from_name,
        from_email: deleted_sender_response.from_email,
    })
}

pub async fn validate_email_identity(
    payload: ValidateEmailIdentityRequest,
) -> Result<ValidateEmailIdentityResponse, (StatusCode, String)> {
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch identities: {}", e),
        )
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
            message: Some(format!(
                "Email '{}' is not verified in AWS SES. Please verify it in the AWS console first.",
                email
            )),
        })
    }
}

// Function to get all verified identities
pub async fn get_verified_identities() -> Result<Vec<String>, (StatusCode, String)> {
    let client = aws_service::create_aws_client().await;
    let request = client.list_email_identities();

    let result = request.send().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch identities: {}", e),
        )
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

pub async fn send_test_email(payload: SendTestEmailRequest) -> Result<SendTestEmailResponse, AppError> {
    let server_repo = Arc::new(ServerRepoImpl);
    let server_service = servers_services::ServerService::new(server_repo);

    // Get server details
    let server = server_service.get_server_by_id(&payload.server_id as &str).await?;

    // Optional: Validate email identity for AWS servers
    let mut validation_message = String::new();
    if server.server_type == ServerTypeEnum::AWS {
        let validation_result = validate_email_identity(ValidateEmailIdentityRequest {
            email: payload.from_email.clone(),
            server_id: payload.server_id.clone(),
        })
        .await;

        match validation_result {
            Ok(result) => {
                if !result.is_valid {
                    validation_message = format!(
                        "Warning: {} ",
                        result
                            .message
                            .unwrap_or_else(|| "Email identity not verified".to_string())
                    );
                }
            }
            Err((_, msg)) => {
                validation_message = format!("Warning: Could not validate email identity. {}", msg);
            }
        }
    }

    // Prepare the subject
    let subject = payload.subject.unwrap_or_else(|| "Test Email".to_string());

    // Send test email based on server type
    let result = match server.server_type {
        ServerTypeEnum::AWS => send_aws_test_email(server, &payload.from_email, &payload.to_email).await,
        ServerTypeEnum::SMTP => send_smtp_test_email(server, &payload.from_email, &payload.to_email).await,
    };

    match result {
        Ok(message) => Ok(SendTestEmailResponse {
            success: true,
            message: if validation_message.is_empty() {
                message
            } else {
                format!("{}. {}", message, validation_message)
            },
        }),
        Err((status, error_message)) => {
            // Convert the tuple error to AppError
            let error_msg = if validation_message.is_empty() {
                error_message
            } else {
                format!("{}. {}", error_message, validation_message)
            };

            match status {
                StatusCode::BAD_REQUEST => Err(AppError::BadRequestError(Some(error_msg))),
                StatusCode::NOT_FOUND => Err(AppError::NotFoundError(Some(error_msg))),
                _ => Err(AppError::InternalServerError(Some(error_msg))),
            }
        }
    }
}

use aws_sdk_sesv2::Client;
pub async fn send_aws_test_email(
    server: Server,
    from_email: &str,
    to_email: &str,
) -> Result<String, (StatusCode, String)> {
    use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
    let client: Client = create_aws_client_db(&server.id.to_string()).await;

    let subject = Content::builder().data("Test Email from AWS SES").build().unwrap();
    let body_text = Content::builder()
        .data("This is a test email to verify AWS SES configuration.")
        .build()
        .unwrap();

    let message = Message::builder()
        .subject(subject)
        .body(Body::builder().text(body_text).build())
        .build();

    let destination = Destination::builder().to_addresses(to_email).build();

    let email_content = EmailContent::builder().simple(message).build();

    let send_result = client
        .send_email()
        .from_email_address(from_email)
        .destination(destination)
        .content(email_content)
        .send()
        .await;

    match send_result {
        Ok(_) => Ok("Test email sent via AWS SES.".to_string()),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send AWS SES email: {}", e),
        )),
    }
}

pub async fn send_smtp_test_email(
    server: Server,
    from_email: &str,
    to_email: &str,
) -> Result<String, (StatusCode, String)> {
    let creds = Credentials::new(server.smtp_username.clone(), server.smtp_password.clone());

    let mailer = SmtpTransport::relay(&server.host.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("SMTP relay error: {}", e)))?
        .port(server.port as u16)
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Test Email from SMTP Server")
        .body("This is a test email to verify SMTP configuration.".to_string())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Build email error: {}", e)))?;

    match mailer.send(&email) {
        Ok(_) => Ok("Test email sent via SMTP.".to_string()),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send SMTP email: {}", e),
        )),
    }
}
