use crate::{models::mail::{DeleteMailResponse, GetMailResponse}, repositories::mail::{self, MailRepository, MailRepositoryImpl}};
use uuid::Uuid;
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::mail::{
    Mail,
    CreateMailRequest,
    CreateMailResponse,
    UpdateMailRequest,
    UpdateMailResponse
};

pub struct MailService {
    repository: Arc<dyn MailRepository + Send + Sync>
}

impl MailService {
    pub fn new(repository: Arc<dyn MailRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_mail(&self, payload: CreateMailRequest) -> Result<Mail, diesel::result::Error> {
        self.repository.create_mail(payload).await
    }

    pub async fn get_all_mails(&self) -> Result<Vec<Mail>, diesel::result::Error> {
        self.repository.get_all_mails().await
    }

    pub async fn update_mail(&self, mail_id: Uuid, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error> {
        self.repository.update_mail(mail_id, payload).await
    }

    pub async fn delete_mail(&self, mail_id: Uuid) -> Result<Mail, diesel::result::Error> {
        self.repository.delete_mail(mail_id).await
    }
}

/// a function to add new mail into the record when the mail_send is triggered...
pub async fn create_mail(payload: CreateMailRequest) -> Result<CreateMailResponse, (StatusCode, String)> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);
    let response = mail_service.create_mail(payload).await;

    // let contact_uuid = Uuid::parse_str(&payload.contact_id).unwrap();

    let response = match response {
        Ok(mail) => CreateMailResponse {
            id: mail.id,
            mail_message: mail.mail_message,
            contact_id: mail.contact_id,
            template_id: mail.template_id,
            campaign_id: mail.campaign_id,
            sent_at: mail.sent_at,
            status: mail.status,
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    Ok(response)
}

/// a function to get all mails from the record...
pub async fn get_all_mails() -> Result<Vec<GetMailResponse>, (StatusCode, String)> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);
    let response = mail_service.get_all_mails().await;

    match response {
        Ok(mails) => Ok(mails.into_iter().map(|mail| GetMailResponse {
            id: mail.id,
            mail_message: mail.mail_message,
            contact_id: mail.contact_id,
            template_id: mail.template_id,
            campaign_id: mail.campaign_id,
            sent_at: mail.sent_at,
            status: mail.status,
        }).collect()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

/// a function to update mail in the record...
pub async fn update_mail(mail_id: Uuid, payload: UpdateMailRequest) -> Result<UpdateMailResponse, (StatusCode, String)> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.update_mail(mail_id, payload).await;

    match response {
        Ok(mail) => Ok(UpdateMailResponse {
            id: mail.id,
            mail_message: mail.mail_message,
            template_id: mail.template_id,
            campaign_id: mail.campaign_id,
            status: Some(mail.status),
            updated_at: chrono::Utc::now(),
        }),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}

/// a function to delete mail from the db relation...
pub async fn delete_mail(mail_id: Uuid) -> Result<DeleteMailResponse, (StatusCode, String)> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.delete_mail(mail_id).await;

    match response {
        Ok(mail) => Ok(DeleteMailResponse {
            id: mail.id,
            status: Some(mail.status),
        }),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}