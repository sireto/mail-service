use crate::error::AppError;
use crate::models::mail::{CreateMailRequest, Mail, UpdateMailRequest, UpdateMailResponse};
use crate::models::pagination::{Page, PageQuery};
use crate::servers::servers_model::ServerTypeEnum;
use crate::servers::servers_services::{ServerService, ServerServiceTrait};
use crate::services::contact_service;
use crate::{
    models::mail::{DeleteMailResponse, MailWithDetails, NewMail},
    repositories::mail_repository::MailRepository,
    services::campaign_service::send_single_email,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
#[cfg(feature = "mocks")]
use mockall::automock;
use std::{collections::HashMap, num::NonZeroU32, sync::Arc};
use tokio::time::{interval, Duration};
use uuid::Uuid;

/// How often the queue is polled, and how often stale claimed mails are retried.
const TICK_SECONDS: u64 = 5;
const RETRY_TICK_SECONDS: u64 = 600;

/// A mail is abandoned after this many send attempts.
pub const MAX_SEND_ATTEMPTS: i32 = 3;

pub const MAIL_STATUS_QUEUED: &str = "queued";
pub const MAIL_STATUS_SUBMITTED: &str = "submitted";
pub const MAIL_STATUS_SENT: &str = "sent";
pub const MAIL_STATUS_FAILED: &str = "failed";
pub const MAIL_STATUS_BOUNCED: &str = "bounced";
pub const MAIL_STATUS_DELIVERED: &str = "delivered";

/// servers.rate_limit is a plain i32 with no database constraint. Casting a negative value
/// straight to u32 wrapped to a huge quota, which silently disabled rate limiting
/// altogether — the opposite of what a misconfigured value should do.
pub fn sanitize_rate_limit(rate_limit: i32) -> NonZeroU32 {
    let clamped = rate_limit.clamp(1, 10_000) as u32;
    NonZeroU32::new(clamped).unwrap_or(NonZeroU32::MIN)
}

pub fn has_reached_send_attempt_limit(attempts: i32) -> bool {
    attempts >= MAX_SEND_ATTEMPTS
}

/// a structure to hold the server state along with its rate limiter and the server_type to decide from what server (either AWS or SMTP) to send the email...
#[derive(Debug)]
pub struct ServerState {
    pub limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    pub server_type: ServerTypeEnum,
    pub rate_limit: i32,
}

#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait MailServiceTrait {
    async fn create_mail(&self, payload: CreateMailRequest) -> Result<Vec<Mail>, AppError>;
    async fn get_mail_by_id(&self, mail_id: String) -> Result<Mail, AppError>;
    async fn get_all_mails(
        &self,
        campaign_ids: Option<Vec<Uuid>>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        page: &PageQuery,
    ) -> Result<Page<MailWithDetails>, AppError>;
    async fn update_mail(&self, mail_id: String, payload: UpdateMailRequest) -> Result<UpdateMailResponse, AppError>;
    async fn update_mail_status(&self, mail_id: String, new_status: &str) -> Result<UpdateMailResponse, AppError>;
    async fn delete_mail(&self, mail_id: String) -> Result<DeleteMailResponse, AppError>;
    async fn increment_mail_clicks(&self, mail_id: String) -> Result<UpdateMailResponse, AppError>;
    async fn get_mails_by_contact(&self, contact_id: Uuid) -> Result<Vec<MailWithDetails>, AppError>;
    async fn fetch_queued_mails(&self) -> Result<Vec<MailWithDetails>, AppError>;
    async fn fetch_bounced_mails(&self) -> Result<Vec<MailWithDetails>, AppError>;
    async fn fetch_stale_submitted_mails(&self) -> Result<Vec<MailWithDetails>, AppError>;
    async fn process_mails(&self, server_service: Arc<ServerService>) -> Result<(), AppError>;
    async fn process_submitted_mails(&self, server_service: Arc<ServerService>) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct MailService {
    repository: Arc<dyn MailRepository + Send + Sync>,
}

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
        for email in payload.email.clone() {
            let contact = contact_service::get_contact_by_email(payload.namespace_id, email).await?;

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

    async fn get_mail_by_id(&self, mail_id: String) -> Result<Mail, AppError> {
        Ok(self.repository.get_mail_by_id(mail_id).await?)
    }

    /// a function to get all mails from the record...
    async fn get_all_mails(
        &self,
        campaign_ids: Option<Vec<Uuid>>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        page: &PageQuery,
    ) -> Result<Page<MailWithDetails>, AppError> {
        let (limit, offset) = (page.limit(), page.offset());
        let (items, total) = self
            .repository
            .get_all_mails(campaign_ids, from, to, limit, offset)
            .await?;

        Ok(Page::new(items, total, limit, offset))
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
        let response = self.repository.get_mails_by_status(MAIL_STATUS_QUEUED, true).await?;

        Ok(response)
    }

    async fn fetch_bounced_mails(&self) -> Result<Vec<MailWithDetails>, AppError> {
        let response = self.repository.get_mails_by_status(MAIL_STATUS_BOUNCED, false).await?;

        Ok(response)
    }

    async fn fetch_stale_submitted_mails(&self) -> Result<Vec<MailWithDetails>, AppError> {
        let response = self.repository.get_stale_submitted_mails().await?;

        Ok(response)
    }

    /// Process queued mails with per-server rate limiting using the governor crate...
    ///
    /// Two properties this loop must preserve, both of which were previously broken:
    ///   - it must not exit. Returning on a transient database error stopped delivery for
    ///     the lifetime of the process, and nothing restarts it.
    ///   - it must pick up servers created after boot. Limiters used to be built once
    ///     before the loop, so a new server's mail was skipped forever.
    /// Limiters are kept across ticks rather than rebuilt, because rebuilding one resets
    /// its token bucket and would defeat the rate limit.
    async fn process_mails(&self, server_service: Arc<ServerService>) -> Result<(), AppError> {
        let mut limiters: HashMap<Uuid, ServerState> = HashMap::new();

        let mut ticker = interval(Duration::from_secs(TICK_SECONDS));
        loop {
            ticker.tick().await;

            match server_service.get_all_servers().await {
                Ok(servers) => Self::sync_limiters(&mut limiters, servers),
                Err(err) => {
                    eprintln!("mail worker: could not refresh servers, reusing previous limiters: {err}");
                }
            }

            let mails = match self.fetch_queued_mails().await {
                Ok(mails) => mails,
                Err(err) => {
                    eprintln!("mail worker: could not fetch queued mails, retrying next tick: {err}");
                    continue;
                }
            };

            if mails.is_empty() {
                continue;
            }

            for mail in mails {
                let Some(sid) = mail.server_id else {
                    eprintln!("mail worker: mail {} has no server_id, skipping", mail.id);
                    continue;
                };

                let Some(campaign_id) = mail.campaign_id else {
                    eprintln!("mail worker: mail {} has no campaign_id, skipping", mail.id);
                    continue;
                };

                let Some(server_state) = limiters.get(&sid) else {
                    eprintln!("mail worker: no server {sid} for mail {}, skipping", mail.id);
                    continue;
                };

                server_state.limiter.until_ready().await;

                // Claim the mail before dispatching so the next tick does not pick it up
                // again and send a duplicate. send_single_email advances it to "sent", or
                // records the failure.
                if let Err(err) = self
                    .repository
                    .update_mail_status(mail.id.clone(), MAIL_STATUS_SUBMITTED)
                    .await
                {
                    eprintln!("mail worker: could not claim mail {}, skipping: {err}", mail.id);
                    continue;
                }

                let repository = Arc::clone(&self.repository);
                let server_type = server_state.server_type;
                let mail_id = mail.id.clone();
                let email = mail.email.clone();
                let message = mail.mail_message.clone();
                let subject = format!("Hello {}", mail.email);

                tokio::spawn(async move {
                    let result =
                        send_single_email(server_type, mail_id.clone(), campaign_id, sid, email, message, subject)
                            .await;

                    // A failure here used to be silently dropped, leaving the mail stuck at
                    // "submitted" and indistinguishable from a success.
                    if let Err(err) = result {
                        eprintln!("mail worker: send failed for mail {mail_id}: {err}");
                        let _ = repository
                            .udpate_mail_last_try_error(mail_id.clone(), &err.to_string())
                            .await;
                        let _ = repository.update_mail_attempts(mail_id).await;
                    }
                });
            }
        }
    }

    /// A process to handle stale submitted mails...
    /// Retries mails that were claimed but never reached a terminal state.
    async fn process_submitted_mails(&self, server_service: Arc<ServerService>) -> Result<(), AppError> {
        let mut ticker = interval(Duration::from_secs(RETRY_TICK_SECONDS));
        loop {
            ticker.tick().await;

            let mails = match self.repository.get_stale_submitted_mails().await {
                Ok(mails) => mails,
                Err(err) => {
                    eprintln!("retry worker: could not fetch stale mails, retrying next tick: {err}");
                    continue;
                }
            };

            for mail in mails {
                if has_reached_send_attempt_limit(mail.attempts) {
                    if let Err(err) = self
                        .repository
                        .update_mail_status(mail.id.clone(), MAIL_STATUS_FAILED)
                        .await
                    {
                        eprintln!("retry worker: could not mark mail {} failed: {err}", mail.id);
                    }
                    continue;
                }

                let Some(sid) = mail.server_id else {
                    continue;
                };

                let server = match server_service.get_server_by_id(&sid.to_string()).await {
                    Ok(server) => server,
                    Err(err) => {
                        eprintln!("retry worker: server {sid} unavailable for mail {}: {err}", mail.id);
                        continue;
                    }
                };

                let Some(campaign_id) = mail.campaign_id else {
                    continue;
                };

                // attempts is incremented on every retry, not only on success. The previous
                // version incremented on success only, so a permanently failing mail never
                // reached MAX_SEND_ATTEMPTS and retried forever.
                if let Err(err) = self.repository.update_mail_attempts(mail.id.clone()).await {
                    eprintln!("retry worker: could not record attempt for mail {}: {err}", mail.id);
                    continue;
                }

                let result = send_single_email(
                    server.server_type,
                    mail.id.clone(),
                    campaign_id,
                    sid,
                    mail.email.clone(),
                    mail.mail_message.clone(),
                    format!("Hello {}", mail.email),
                )
                .await;

                if let Err(err) = result {
                    eprintln!("retry worker: retry failed for mail {}: {err}", mail.id);
                    let _ = self
                        .repository
                        .udpate_mail_last_try_error(mail.id.clone(), &err.to_string())
                        .await;

                    // The increment happened before this send. Finalize the row now when
                    // this was its last allowed attempt rather than leaving it submitted
                    // until another retry tick.
                    if has_reached_send_attempt_limit(mail.attempts.saturating_add(1)) {
                        if let Err(status_err) = self
                            .repository
                            .update_mail_status(mail.id.clone(), MAIL_STATUS_FAILED)
                            .await
                        {
                            eprintln!("retry worker: could not mark mail {} failed: {status_err}", mail.id);
                        }
                    }
                }
            }
        }
    }
}

impl MailService {
    /// Add limiters for servers we have not seen and drop limiters for servers that are
    /// gone, without disturbing the token bucket of a server that is unchanged.
    fn sync_limiters(limiters: &mut HashMap<Uuid, ServerState>, servers: Vec<crate::servers::servers_model::Server>) {
        let mut seen = Vec::with_capacity(servers.len());

        for server in servers {
            seen.push(server.id);
            let per_sec = sanitize_rate_limit(server.rate_limit);

            match limiters.get_mut(&server.id) {
                // Only rebuild when the configured rate actually changed, since rebuilding
                // resets the bucket.
                Some(existing) if existing.rate_limit == server.rate_limit => {
                    existing.server_type = server.server_type;
                }
                _ => {
                    limiters.insert(
                        server.id,
                        ServerState {
                            limiter: RateLimiter::direct(Quota::per_second(per_sec)),
                            server_type: server.server_type,
                            rate_limit: server.rate_limit,
                        },
                    );
                }
            }
        }

        limiters.retain(|id, _| seen.contains(id));
    }
}

/// Process exactly one batch of queued mails (no loop or ticker).
/// the function for testing...
pub async fn process_one_batch(
    servers: &HashMap<Uuid, ServerState>,                   // server details...
    fetch: impl Fn() -> Vec<MailWithDetails> + Send + Sync, // fetch all the mailDetails...
    send: impl Fn(&MailWithDetails) + Send + Sync,          // send the emails...
    update: impl Fn(String) + Send + Sync,                  // update the mail status...
) {
    for mail in fetch() {
        let Some(server_id) = mail.server_id else { continue };
        if let Some(state) = servers.get(&server_id) {
            state.limiter.until_ready().await;
            send(&mail);
            update(mail.id.clone());
        }
    }
}
