use crate::{handlers::campaign, models::mail::{DeleteMailResponse, GetMailResponse, MailWithDetails, NewMail}, repositories::mail_repository::{self, MailRepository, MailRepositoryImpl}};
use crate::services::contact as contact_service;
use chrono::{DateTime, Utc};
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

use super::bounce_logs_service;

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
}

/// a function to add new mail into the record when the mail_send is triggered...
pub async fn create_mail(payload: CreateMailRequest) -> Result<Vec<CreateMailResponse>, (StatusCode, String)> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);
    println!("PAYLOAD EMAIL: {:?}", payload.email);
    // let contact_uuid = Uuid::parse_str(&payload.contact_id).unwrap();
    let mut responses = Vec::new(); // Vec<CreateMailResponse>;
    for email in payload.email {
        let contact = contact_service::get_contact_by_email(email).await?;

        let new_mail = NewMail {
            id: payload.id.clone(),
            mail_message: payload.mail_message.clone(),
            contact_id: contact.id,
            template_id: payload.template_id,
            campaign_id: payload.campaign_id,
            sent_at: payload.sent_at,
            status: payload.status.clone(),
        };

        let response = mail_service.create_mail(new_mail).await;

        println!("THE RESPONSE MAIL ====> {response:?}");
        println!("AFTER AFTER ADDING TO THE MAIL");

        match response {
            Ok(mail) => responses.push(CreateMailResponse {
                id: mail.id,
                mail_message: mail.mail_message,
                contact_id: mail.contact_id,
                template_id: mail.template_id,
                campaign_id: mail.campaign_id,
                sent_at: mail.sent_at,
                status: mail.status,
            }),
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            }
        };
    }

    Ok(responses)
}

/// a function to get all mails from the record...
pub async fn get_all_mails(
    campaign_ids: Option<Uuid>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>
) -> Result<Vec<MailWithDetails>, (StatusCode, String)> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);
    let response = mail_service.get_all_mails(campaign_ids, from, to).await;

    match response {
        Ok(mails) => Ok(mails),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

/// a function to update mail in the record...
pub async fn update_mail(mail_id: String, payload: UpdateMailRequest) -> Result<UpdateMailResponse, (StatusCode, String)> {
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

/// a function to update mail status...
pub async fn update_mail_status(mail_id: String, new_status: String) -> Result<UpdateMailResponse, (StatusCode, String)> {
    let mail_repository = Arc::new(MailRepositoryImpl);
    let mail_service = MailService::new(mail_repository);

    let response = mail_service.update_mail_status(mail_id, &new_status).await;

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
pub async fn delete_mail(mail_id: String) -> Result<DeleteMailResponse, (StatusCode, String)> {
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