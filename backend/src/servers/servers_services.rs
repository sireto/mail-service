use crate::{error::AppError, schema::sql_types::TlsType, servers::servers_repo::{ServerRepo, ServerRepoImpl}};

use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::servers::servers_model::{Server, ServerRequest};
use async_trait::async_trait;

use super::servers_model::TlsTypeEnum;

use lettre::{
    message::{Message, MultiPart, SinglePart}, transport::smtp::{authentication::Credentials, client::{Tls, TlsParameters}}, SmtpTransport, Transport
};

#[async_trait]
pub trait ServerServiceTrait {
    async fn create_server(&self, payload: ServerRequest) -> Result<Server, AppError>;
    async fn get_all_servers(&self) -> Result<Vec<Server>, AppError>;
    async fn get_server_by_id(&self, server_id: &str) -> Result<Server, AppError>;
    async fn update_server(&self, server_id: &str, payload: ServerRequest) -> Result<Server, AppError>;
    async fn delete_server(&self, server_id: &str) -> Result<Server, AppError>;
    async fn send_mail_with_smtp(
        &self,
        server_id: Uuid,
        from: &str,
        to: Vec<String>,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        subject: &str,
        html_data: &str,
    ) -> Result<String, (StatusCode, String)>;
    async fn check_credentials(&self, server_id: &str) -> Result<(), AppError>;
}
#[derive(Clone)]
pub struct ServerService {
    repository: Arc<dyn ServerRepo + Send + Sync>
}

impl ServerService {
    pub fn new(repository: Arc<dyn ServerRepo + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ServerServiceTrait for ServerService {

     async fn create_server(&self, payload: ServerRequest) -> Result<Server, AppError> {
        let server = self.repository.create_server(payload).await?;

        Ok(server)
    }

     async fn get_all_servers(&self) -> Result<Vec<Server>, AppError> {
        let servers = self.repository.get_all_servers().await?;

        Ok(servers)
    }

     async fn get_server_by_id(&self, server_id: &str) -> Result<Server, AppError> {
        let uuid_id = Uuid::parse_str(server_id)?;

        let server = self.repository.get_server_by_id(uuid_id).await?;

        Ok(server)
    }

     async fn update_server(&self, server_id: &str, payload: ServerRequest) -> Result<Server, AppError> {
        let uuid_id = Uuid::parse_str(server_id)?;

        let updated_server = self.repository.update_server(uuid_id, payload).await?;

        Ok(updated_server)
    }

     async fn delete_server(&self, server_id: &str) -> Result<Server, AppError> {
        let uuid_id = Uuid::parse_str(server_id)?;

        let deleted_server = self.repository.delete_server(uuid_id).await?;

        Ok(deleted_server)
    }

    async fn send_mail_with_smtp(
        &self,
        server_id: Uuid,
        from: &str,
        to: Vec<String>,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        subject: &str,
        html_data: &str,
    ) -> Result<String, (StatusCode, String)> {

        // Fetch the server details
        let server = self.repository.get_server_by_id(server_id).await
            .map_err(|err| (StatusCode::NOT_FOUND, format!("Server not found: {}", err)))?;

            println!("SMTP Host: {}, Username: {}, PASSWORD: {}", server.host, server.smtp_username, server.smtp_password);


        let credentials = Credentials::new(server.smtp_username.clone(), server.smtp_password.clone());

        let tls_parameters = TlsParameters::new(server.host.clone())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create TLS parameters: {}", e)))?;

        let mailer = match server.tls_type {
            crate::servers::servers_model::TlsTypeEnum::STARTTLS => {
                SmtpTransport::relay(&server.host)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create SMTP transport: {}", e)))?
                    .port(server.port as u16)
                    .credentials(credentials)
                    .tls(Tls::Required(tls_parameters))
                    .build()
            },
            crate::servers::servers_model::TlsTypeEnum::SSLTLS => {
                SmtpTransport::relay(&server.host)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create SMTP transport: {}", e)))?
                    .port(server.port as u16)
                    .credentials(credentials)
                    .tls(Tls::Wrapper(tls_parameters))
                    .build()
            },
            crate::servers::servers_model::TlsTypeEnum::NONE => {
                SmtpTransport::relay(&server.host)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create SMTP transport: {}", e)))?
                    .port(server.port as u16)
                    .credentials(credentials)
                    .tls(Tls::None)
                    .build()
            },
        };

        let mut email_builder = Message::builder()
            .from(from.parse().map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid sender email: {}", e)))?)
            .subject(subject);

        for recipient in to {
            email_builder = email_builder.to(recipient.parse().map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid recipient email: {}", e)))?);
        }

        if let Some(cc_list) = cc {
            for cc_recipient in cc_list {
                email_builder = email_builder.cc(cc_recipient.parse().map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid CC email: {}", e)))?);
            }
        }

        if let Some(bcc_list) = bcc {
            for bcc_recipient in bcc_list {
                email_builder = email_builder.bcc(bcc_recipient.parse().map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid BCC email: {}", e)))?);
            }
        }

        let email = email_builder
            .multipart(
                MultiPart::alternative()
                    .singlepart(SinglePart::html(html_data.to_string()))
            )
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to build email: {}", e)))?;

        match mailer.send(&email) {
            Ok(_) => Ok("Email sent successfully".to_string()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to send email: {}", e))),
        }
    }

    async fn check_credentials(&self, server_id: &str) -> Result<(), AppError>{
        let uuid_id = Uuid::parse_str(server_id)
            .map_err(|e| AppError::UuidError((e)))?;

        let server = self.repository.get_server_by_id(uuid_id).await?;

        let credentials = Credentials::new(server.smtp_username.clone(), server.smtp_password.clone());

        let tls_parameters = TlsParameters::new(server.host.clone())
            .map_err(|e| AppError::InternalServerError(Some(e.to_string())))?;

        let mailer = match server.tls_type {
            TlsTypeEnum::STARTTLS => {
                SmtpTransport::relay(&server.host)
                .map_err(|e| AppError::InternalServerError(Some(format!("Failed to establish connection: {}", e))))?

                .port(server.port as u16)
                .credentials(credentials)
                .tls(Tls::Required(tls_parameters))
                .build()
            }
            TlsTypeEnum::SSLTLS => {
                SmtpTransport::relay(&server.host)
                .map_err(|e| AppError::InternalServerError(Some(format!("Failed to establish connection: {}", e))))?
                .port(server.port as u16)
                .credentials(credentials)
                .tls(Tls::Wrapper(tls_parameters))
                .build()
            }
            TlsTypeEnum::NONE => {
                SmtpTransport::relay(&server.host)
                    .map_err(|e| AppError::InternalServerError(Some(format!("Failed to establish connection: {}", e))))?
                    
                .port(server.port as u16)
                .credentials(credentials)
                .tls(Tls::None)
                .build()
            }
        };
        let _conn = mailer.test_connection().map_err(|e| AppError::SmtpError(e))?;

        Ok(())
    }
}

