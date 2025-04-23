use crate::{error::AppError, schema::sql_types::TlsType, models::contact::Contact, servers::{servers_repo::{ServerRepo, ServerRepoImpl}, servers_services}};

use uuid::Uuid;
use std::sync::Arc;
use axum::extract::Extension;
use axum::http::StatusCode;
use crate::models::mail::{CreateMailRequest, MailWithDetails};
use crate::services::mail_service;
use crate::servers::servers_model::{ Server, ServerRequest, SendMailFromServerResponse, ServerTypeEnum };
use crate::services::template_service::{get_template_by_id, send_templated_email};
use crate::models::template::SendMailRequest;
use crate::models::contact::GetContactResponse;
use crate::services::contact_service::get_contact_by_email;
use crate::utils::contact_lists_functions::populate_contact_template;
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
    async fn check_credentials(&self, payload: ServerRequest) -> Result<(), AppError>;
    async fn send_mail_from_server(&self, server_id: &str, template_id: Uuid, receiver: String) -> Result<SendMailFromServerResponse, AppError>;
    async fn get_mails_by_server_id(&self, server_id: Uuid) -> Result<Vec<MailWithDetails>, AppError>;
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

    async fn check_credentials(&self, payload: ServerRequest) -> Result<(), AppError>{

        let credentials = Credentials::new(payload.smtp_username.clone(), payload.smtp_password.clone());

        let tls_parameters = TlsParameters::new(payload.host.clone())
            .map_err(|e| AppError::InternalServerError(Some(e.to_string())))?;

        let mailer = match payload.tls_type {
            TlsTypeEnum::STARTTLS => {
                SmtpTransport::relay(&payload.host)
                .map_err(|e| AppError::InternalServerError(Some(format!("Failed to establish connection: {}", e))))?

                .port(payload.port as u16)
                .credentials(credentials)
                .tls(Tls::Required(tls_parameters))
                .build()
            }
            TlsTypeEnum::SSLTLS => {
                SmtpTransport::relay(&payload.host)
                .map_err(|e| AppError::InternalServerError(Some(format!("Failed to establish connection: {}", e))))?
                .port(payload.port as u16)
                .credentials(credentials)
                .tls(Tls::Wrapper(tls_parameters))
                .build()
            }
            TlsTypeEnum::NONE => {
                SmtpTransport::relay(&payload.host)
                    .map_err(|e| AppError::InternalServerError(Some(format!("Failed to establish connection: {}", e))))?
                    
                .port(payload.port as u16)
                .credentials(credentials)
                .tls(Tls::None)
                .build()
            }
        };
        let _conn = mailer.test_connection().map_err(|e| AppError::SmtpError(e))?;

        Ok(())
    }

    async fn send_mail_from_server(
        &self, 
        server_id: &str, 
        template_id: Uuid, 
        receiver: String
    ) -> Result<SendMailFromServerResponse, AppError> {
        let uuid_id = Uuid::parse_str(server_id).map_err(|err| AppError::BadRequestError(Some(format!("Invalid server_id format: {}", err))))?;

        let server = self.repository.get_server_by_id(uuid_id).await?;

        let receivers = receiver.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
        let mut mail_send_ids: Vec<Uuid> = Vec::new();

        match server.server_type {
            // send mail using the AWS credentials...
            ServerTypeEnum::AWS => {
                let mail_send_uuid = Uuid::new_v4();
                let mail_send_id = mail_send_uuid.to_string();
                let payload = SendMailRequest {
                    receiver: Some(receiver.clone()),
                    cc: None,
                    bcc: None,
                    from: server.default_from_email,
                    subject: "Test Subject".to_string(),
                    template_data: "{\"first_name\":\"John\", \"last_name\":\"Doe\",\"email\":\"john@gmail.com\"}".to_string(),
                };

                let mail_sent = send_templated_email(template_id, payload).await.map_err(|err| AppError::InternalServerError(Some(format!("Failed to send email: {}", err))))?;   

                // Create mail record after sending
                let new_mail = CreateMailRequest {
                    id: mail_send_id,
                    mail_message: mail_sent.message, // or store template_data / generated HTML
                    email: vec![receiver],
                    template_id: Some(template_id),
                    campaign_id: None, // fill if available
                    server_id: Some(server.id),
                    sent_at: chrono::Utc::now(),
                    status: "sent".to_string(),
                };
            
                mail_service::create_mail(new_mail).await.map_err(|err| {
                    AppError::InternalServerError(Some(format!("Failed to create mail: {}", err)))
                })?;

                Ok(SendMailFromServerResponse {
                    id: mail_send_uuid,
                    template_id,
                    server_id: server.id,
                    mail_send_ids: vec![mail_send_uuid],
                    status: "queued".to_string(),
                    sent_at: chrono::Utc::now(),
                }) 
            },
            // send mail using SMTP server...
            ServerTypeEnum::SMTP => {
                // get the template...
                let template = get_template_by_id(template_id).await
                .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

                // populate the template with data...
                for email in &receivers {
                    let mail_send_uuid = Uuid::new_v4();
                    let mail_send_id = mail_send_uuid.to_string();
                    let contact_response = get_contact_by_email(email.clone()).await
                    .map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

                    // transform the contact_response to the contact object as the populate_contact_template() only accepts the <Contact>...
                    let contact = Contact {
                        id: contact_response.id,
                        first_name: contact_response.first_name,
                        last_name: contact_response.last_name,
                        email: contact_response.email,
                        attribute: contact_response.attribute,
                        created_at: contact_response.created_at,
                        updated_at: contact_response.updated_at,
                    };

                    let populated_template = populate_contact_template(&template, &contact).await
                    .map_err(|err| AppError::InternalServerError(Some(format!("Error populating the template: {}", err.to_string()))))?;

                    let _mail_sent = self.send_mail_with_smtp(
                        uuid_id,
                        server.default_from_email.as_str(),
                        vec![contact.email.clone()],
                        None,
                        None,
                        &format!("Hello {}! Test Subject", contact.first_name),
                        populated_template.as_str(),
                    ).await.map_err(|err| AppError::InternalServerError(Some(format!("Failed to send email: {}", err.1))))?;    // access the error message from the tuple...

                    // Create mail record after sending...
                    let new_mail = CreateMailRequest {
                        id: mail_send_id,
                        mail_message: populated_template, // or store template_data / generated HTML...
                        email: vec![contact.email.clone()],
                        template_id: Some(template_id),
                        campaign_id: None,
                        server_id: Some(server.id),
                        sent_at: chrono::Utc::now(),
                        status: "queued".to_string(),
                    };
                
                    mail_service::create_mail(new_mail).await.map_err(|err| {
                        AppError::InternalServerError(Some(format!("Failed to create mail: {}", err)))
                    })?;

                    mail_send_ids.push(mail_send_uuid);
                }
                
                Ok(SendMailFromServerResponse {
                    id: uuid_id,
                    template_id,
                    server_id: server.id,
                    mail_send_ids,
                    status: "sent".to_string(),
                    sent_at: chrono::Utc::now(),
                })
            }
        }
    }

    async fn get_mails_by_server_id(&self, server_id: Uuid) -> Result<Vec<MailWithDetails>, AppError> {
        let mails = self.repository.get_mails_by_server_id(server_id).await?;

        Ok(mails)
    }
}

