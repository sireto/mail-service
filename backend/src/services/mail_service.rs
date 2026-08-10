use crate::{models::mail::{DeleteMailResponse, MailWithDetails, NewMail}, repositories::mail_repository::MailRepository, services::campaign_service::send_single_email};
use crate::servers::servers_services::{ ServerService, ServerServiceTrait };
use crate::services::contact_service as contact_service;
use chrono::{DateTime, Utc};
use tokio::time::{interval, Duration};
use uuid::Uuid;
use std::{collections::HashMap, num::NonZeroU32, sync::Arc, time::Instant};
use crate::models::mail::{
    Mail,
    CreateMailRequest,
    UpdateMailRequest,
    UpdateMailResponse
};
use crate::error::AppError;
use governor::{Quota, RateLimiter, clock::DefaultClock, state::{ InMemoryState, NotKeyed }, middleware::NoOpMiddleware};
use crate::servers::servers_model::ServerTypeEnum;
use async_trait::async_trait;
use mockall::{ automock, predicate::* };


/// a structure to hold the server state along with its rate limiter and the server_type to decide from what server (either AWS or SMTP) to send the email...
#[derive(Debug)]
pub struct ServerState {
    pub limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    pub server_type: ServerTypeEnum,
}

#[automock]
#[async_trait]
pub trait MailServiceTrait {
    async fn create_mail(&self, payload: CreateMailRequest) -> Result<Vec<Mail>, AppError>;
    async fn get_all_mails(&self, campaign_ids: Option<Uuid>, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> Result<Vec<MailWithDetails>, AppError>;
    async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<UpdateMailResponse, AppError>;
    async fn update_mail_status(&self, mail_id: String, new_status: &str) -> Result<UpdateMailResponse, AppError>;
    async fn delete_mail(&self, mail_id: String) -> Result<DeleteMailResponse, AppError>;
    async fn increment_mail_clicks(&self, mail_id: String) -> Result<UpdateMailResponse, AppError>;
    async fn get_mails_by_contact(&self, contact_id: Uuid) -> Result<Vec<MailWithDetails>, AppError>;
    async fn fetch_queued_mails(&self) -> Result<Vec<MailWithDetails>, AppError>;
    async fn fetch_bounced_mails(&self) -> Result<Vec<MailWithDetails>, AppError>;
    async fn fetch_stale_submitted_mails(&self) -> Result<Vec<MailWithDetails>, AppError>;
    async fn process_mails(
        &self,
        server_service: Arc<ServerService>,
    ) -> Result<(), AppError>;
    async fn process_submitted_mails(
        &self,
        server_service: Arc<ServerService>,
    ) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct MailService {
    repository: Arc<dyn MailRepository + Send + Sync>
}

#[automock]
impl MailService {
    pub fn new(repository: Arc<dyn MailRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl MailServiceTrait for MailService {
    /// a function to add new mail into the record when the mail_send is triggered...
    async fn create_mail(&self, payload: CreateMailRequest) -> Result<Vec<Mail>, AppError> {    
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
            let response = self.repository.create_mail(new_mail).await?;

            responses.push(response);
        }

        Ok(responses)
    }

    /// a function to get all mails from the record...
    async fn get_all_mails(
        &self,
        campaign_ids: Option<Uuid>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>
    ) -> Result<Vec<MailWithDetails>, AppError> {    
        let response = self.repository.get_all_mails(campaign_ids, from, to).await?;

        Ok(response)
    }

    /// a function to update mail in the record...
    async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<UpdateMailResponse, AppError> {
        let response = self.repository.update_mail(mail_id, payload).await?;

        let updated_mail_response: UpdateMailResponse = response.into();

        Ok(updated_mail_response)
    }

    /// a function to update mail status...
    async fn update_mail_status(&self, mail_id: String, new_status: &str) -> Result<UpdateMailResponse, AppError> {
        let response = self.repository.update_mail_status(mail_id, new_status).await?;

        let updated_mail_response: UpdateMailResponse = response.into();

        Ok(updated_mail_response)
    }

    /// a function to delete mail from the db relation...
    async fn delete_mail(&self, mail_id: String) -> Result<DeleteMailResponse, AppError> {
        let response = self.repository.delete_mail(mail_id).await?;

        Ok(DeleteMailResponse {
            id: response.id,
            status: Some(response.status),
            contact_id: Some(response.contact_id),
        })
    }

    async fn increment_mail_clicks(&self, mail_id: String) -> Result<UpdateMailResponse, AppError> {
        let response = self.repository.increment_mail_clicks(mail_id).await?;

        let updated_mail_response: UpdateMailResponse = response.into();

        Ok(updated_mail_response)
    }

    async fn get_mails_by_contact(&self, contact_id: Uuid) -> Result<Vec<MailWithDetails>, AppError> {
        let response = self.repository.get_mails_by_contact(contact_id).await?;

        Ok(response)
    }

    async fn fetch_queued_mails(&self) -> Result<Vec<MailWithDetails>, AppError> {
        let response = self.repository.get_mails_by_status("queued", true).await?;

        Ok(response)
    }

    async fn fetch_bounced_mails(&self) -> Result<Vec<MailWithDetails>, AppError> {
        let response = self.repository.get_mails_by_status("bounced", false).await?;

        Ok(response)
    }

    async fn fetch_stale_submitted_mails(&self) -> Result<Vec<MailWithDetails>, AppError> {
        let response = self.repository.get_stale_submitted_mails().await?;

        Ok(response)
    }

    /// Process queued mails with per-server rate limiting using the governor crate...
    async fn process_mails(
        &self,
        server_service: Arc<ServerService>,
    ) -> Result<(), AppError> {
        // Create a rate limiter for each server...
        let mut limiters: HashMap<Uuid, ServerState> = HashMap::new();
        for server in server_service.get_all_servers().await? {
            // rate_limit defines max tokens per second
            let per_sec = NonZeroU32::new(server.rate_limit as u32)
                .unwrap_or_else(|| NonZeroU32::new(1).unwrap());
            let quota = Quota::per_second(per_sec);

            let limiter = RateLimiter::direct(quota);

            let server_state = ServerState {
                limiter,
                server_type: server.server_type,
            };

            limiters.insert(server.id, server_state);
        }

        // run the process in a loop each second...
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;


            let mails = self.fetch_queued_mails().await?;

            if mails.is_empty() {
                println!("No queued mails to process");
                continue;
            }
            for mail in mails {
                let sid = match mail.server_id {
                    Some(id) => id,
                    None => continue,
                };

                if let Some(server_state) = limiters.get(&sid) {
                    println!("[Server {:?}] waiting for token to send mail {} to {}", sid, mail.id, mail.email);
                    let start = Instant::now();
                    // try by adding the until_ready() to the limiter...
                    server_state.limiter.until_ready().await;
                    let waited = start.elapsed();
                    println!("[Server {:?}] waited {:?} before sending mail {}", sid, waited, mail.id);
                    let email = mail.email.clone();

                    // create n background task to send the email bound by server rate limit...
                    tokio::spawn(
                        send_single_email(
                            server_state.server_type,
                            mail.id.clone(),
                            mail.campaign_id.unwrap(),
                            sid,
                            email,
                            mail.mail_message.clone(),
                            format!("Hello {}", mail.email),
                        )
                    );

                    // Update status on success
                    self.repository.update_mail_status(mail.id, "submitted").await?;
                }
            }
        }
    }

    /// A process to handle stale submitted mails...
    /// This function will retry sending mails that are stuck in the submitted state...
    async fn process_submitted_mails(
        &self,
        server_service: Arc<ServerService>,
    ) -> Result<(), AppError> {
        let mut ticker = interval(Duration::from_secs(600)); // Retry every 10 minutes or longer...
        loop {
            ticker.tick().await;
    
            let mails = self.repository.get_stale_submitted_mails().await?;
    
            if mails.is_empty() {
                println!("No submitted mails to retry");
                continue;
            }
    
            for mail in mails {
                if mail.attempts >= 3 {
                    println!("Mail {} exceeded max retry attempts, marking as failed. No more retries will be done for this mail", mail.id);
                    self.repository.update_mail_status(mail.id, "failed").await?;
                    continue;
                }

                let sid = match mail.server_id {
                    Some(id) => id,
                    None => continue,
                };

                let server = server_service.get_server_by_id(&sid.to_string()).await?;
    
                println!("Retrying submitted mail {}", mail.id);
    
                // Retry send (optionally with exponential backoff)
                let result = send_single_email(
                    server.server_type,
                    mail.id.clone(),
                    mail.campaign_id.unwrap(),
                    sid,
                    mail.email.clone(),
                    mail.mail_message.clone(),
                    format!("Hello {}", mail.email),
                ).await;
    
                if result.is_ok() {
                    println!("Mail {} retried successfully", mail.id);
                    self.repository.update_mail_attempts(mail.id).await?;
                } else {
                    println!("Mail {} retry failed", mail.id);
                    self.repository.udpate_mail_last_try_error(
                        mail.id, 
                        "Several retries to send mail failed",
                    ).await?;
                }
            }
        }
    } 
}

/// Process exactly one batch of queued mails (no loop or ticker).
/// the function for testing...
pub async fn process_one_batch(
    servers: &HashMap<Uuid, ServerState>,   // server details...
    fetch: impl Fn() -> Vec<MailWithDetails> + Send + Sync, // fetch all the mailDetails...
    send: impl Fn(&MailWithDetails) + Send + Sync,  // send the emails...
    update: impl Fn(String) + Send + Sync,  // update the mail status...
) {
    for mail in fetch() {
        if let Some(state) = servers.get(&mail.server_id.unwrap()) {
            state.limiter.until_ready().await;
            send(&mail);
            update(mail.id.clone());
        }
    }
}