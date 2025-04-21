use crate::{models::mail::{DeleteMailResponse, MailWithDetails, NewMail}, repositories::mail_repository::{ MailRepository, MailRepositoryImpl }};
use crate::services::contact_service as contact_service;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;
use crate::models::mail::{
    Mail,
    CreateMailRequest,
    UpdateMailRequest,
    UpdateMailResponse
};
use crate::error::AppError;

pub struct MailService {
    repository: Arc<dyn MailRepository + Send + Sync>
}

impl MailService {
    pub fn new(repository: Arc<dyn MailRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn create_mail(&self, payload: NewMail) -> Result<Mail, diesel::result::Error> {
        self.repository.create_mail(payload).await
    }

    pub async fn get_all_mails(&self, campaign_ids: Option<Uuid>, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        self.repository.get_all_mails(campaign_ids, from, to).await
    }

    pub async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<Mail, diesel::result::Error> {
        self.repository.update_mail(mail_id, payload).await
    }

    pub async fn update_mail_status(&self, mail_id: String, new_status: &str) -> Result<Mail, diesel::result::Error> {
        self.repository.update_mail_status(mail_id, new_status).await
    }

    pub async fn delete_mail(&self, mail_id: String) -> Result<Mail, diesel::result::Error> {
        self.repository.delete_mail(mail_id).await
    }

    pub async fn increment_mail_clicks(&self, mail_id: String) -> Result<Mail, diesel::result::Error> {
        self.repository.increment_mail_clicks(mail_id).await
    }

    pub async fn get_mails_by_contact(&self, contact_id: Uuid) -> Result<Vec<MailWithDetails>, diesel::result::Error> {
        self.repository.get_mails_by_contact(contact_id).await
    }
}

/// a function to add new mail into the record when the mail_send is triggered...
pub async fn create_mail(payload: CreateMailRequest) -> Result<Vec<Mail>, AppError> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);
    
    let mut responses = Vec::new(); // Vec<CreateMailResponse>;
    for email in payload.email {
        let contact = contact_service::get_contact_by_email(email).await?;

        let new_mail = NewMail {
            id: payload.id.clone(),
            mail_message: payload.mail_message.clone(),
            contact_id: contact.id,
            template_id: payload.template_id,
            campaign_id: payload.campaign_id,
            server_id: payload.server_id,
            sent_at: payload.sent_at,
            status: payload.status.clone(),
        };
        let response = mail_service.create_mail(new_mail).await?;

        responses.push(response);
    }

    Ok(responses)
}

/// a function to get all mails from the record...
pub async fn get_all_mails(
    campaign_ids: Option<Uuid>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>
) -> Result<Vec<MailWithDetails>, AppError> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);
    
    let response = mail_service.get_all_mails(campaign_ids, from, to).await?;

    Ok(response)
}

/// a function to update mail in the record...
pub async fn update_mail(mail_id: String, payload: UpdateMailRequest) -> Result<UpdateMailResponse, AppError> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.update_mail(mail_id, payload).await?;

    Ok(UpdateMailResponse {
        id: response.id,
        mail_message: response.mail_message,
        template_id: response.template_id,
        campaign_id: response.campaign_id,
        status: Some(response.status),
        updated_at: chrono::Utc::now(),
        open: response.open,
        clicks: response.clicks,
    })
}

/// a function to update mail status...
pub async fn update_mail_status(mail_id: String, new_status: String) -> Result<UpdateMailResponse, AppError> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.update_mail_status(mail_id, &new_status).await?;

    Ok(UpdateMailResponse {
        id: response.id,
        mail_message: response.mail_message,
        template_id: response.template_id,
        campaign_id: response.campaign_id,
        status: Some(response.status),
        updated_at: chrono::Utc::now(),
        open: response.open,
        clicks: response.clicks,
    })
}

/// a function to delete mail from the db relation...
pub async fn delete_mail(mail_id: String) -> Result<DeleteMailResponse, AppError> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.delete_mail(mail_id).await?;

    Ok(DeleteMailResponse {
        id: response.id,
        status: Some(response.status),
    })
}

pub async fn increment_mail_clicks(mail_id: String) -> Result<UpdateMailResponse, AppError> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.increment_mail_clicks(mail_id).await?;

    Ok(UpdateMailResponse {
        id: response.id,
        mail_message: response.mail_message,
        template_id: response.template_id,
        campaign_id: response.campaign_id,
        status: Some(response.status),
        updated_at: chrono::Utc::now(),
        open: response.open,
        clicks: response.clicks,
    })
}

pub async fn get_mails_by_contact(contact_id: Uuid) -> Result<Vec<MailWithDetails>, AppError> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.get_mails_by_contact(contact_id).await?;

    Ok(response)
}