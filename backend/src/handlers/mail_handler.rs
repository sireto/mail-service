use crate::models::mail::{
    CreateMailRequest, CreateMailResponse, DeleteMailResponse, GetMailResponse, MailQuery, UpdateMailRequest,
    UpdateMailResponse,
};
use crate::services::mail_service::{MailService, MailServiceTrait};

use crate::error::AppError;
use crate::models::pagination::{Page, PageQuery};
use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use std::sync::Arc;

/// Statuses a caller may record directly. Deliberately excludes "queued", which is what
/// the worker acts on.
const RECORDABLE_STATUSES: [&str; 4] = ["draft", "sent", "failed", "bounced"];

fn is_recordable_status(status: &str) -> bool {
    let status = status.trim().to_lowercase();
    RECORDABLE_STATUSES.contains(&status.as_str())
}

#[utoipa::path(
    post,
    path = "/api/mails",
    responses(
        (status = 200, description = "Add a new mail in record", body = Vec<CreateMailRequest>),
        (status = 404)
    )
)]
pub async fn add_mail(
    Extension(mail_service): Extension<Arc<MailService>>,
    Json(mut payload): Json<CreateMailRequest>,
) -> Result<Json<Vec<CreateMailResponse>>, AppError> {
    // This endpoint is a direct insert into the mails table, which the background worker
    // drains. Accepting a client-supplied status let any caller enqueue arbitrary HTML to
    // any known contact and have it delivered with the account's own sending credentials.
    // Enqueuing is the campaign endpoints' job; this one only records.
    if !is_recordable_status(&payload.status) {
        return Err(AppError::BadRequestError(Some(format!(
            "status must be one of {}; use the campaign send endpoints to queue mail",
            RECORDABLE_STATUSES.join(", ")
        ))));
    }
    payload.status = payload.status.trim().to_lowercase();

    let created_mail = mail_service.create_mail(payload).await?;

    let mut responses = Vec::new();

    for mail in created_mail {
        responses.push(CreateMailResponse {
            id: mail.id,
            mail_message: mail.mail_message,
            contact_id: mail.contact_id,
            template_id: mail.template_id,
            campaign_id: mail.campaign_id,
            sent_at: mail.sent_at,
            status: mail.status,
            server_id: mail.server_id,
        });
    }
    Ok(Json(responses))
}

#[utoipa::path(
    get,
    path = "/api/mails/",
    params(MailQuery, PageQuery),
    responses(
        (status = 200, description = "A page of mails", body = Page<GetMailResponse>),
        (status = 404)
    )
)]
pub async fn get_all_mails(
    Extension(mail_service): Extension<Arc<MailService>>,
    query: Query<MailQuery>,
    Query(page): Query<PageQuery>,
) -> Result<Json<Page<GetMailResponse>>, AppError> {
    let all_mails = mail_service
        .get_all_mails(query.parsed_campaign_ids()?, query.from, query.to, &page)
        .await?;

    let mut responses = Vec::new();

    all_mails.items.iter().for_each(|mail| {
        responses.push(GetMailResponse {
            id: mail.id.clone(),
            mail_message: mail.mail_message.clone(),
            email: mail.email.clone(),
            template_id: mail.template_id,
            campaign_id: mail.campaign_id,
            sent_at: mail.sent_at,
            open: mail.open,
            clicks: mail.clicks,
            status: mail.status.clone(),
            status_reason: mail.reason.clone(),
            server_id: mail.server_id,
            scheduled_at: mail.scheduled_at,
            attempts: mail.attempts,
            last_error: mail.last_error.clone(),
            from_name: mail.from_name.clone(),
        });
    });
    Ok(Json(all_mails.with_items(responses)))
}

#[utoipa::path(
    patch,
    path = "/api/mails/{mail_id}",
    responses(
        (status = 200, description = "Update a mail", body = UpdateMailResponse),
        (status = 404)
    )
)]
pub async fn update_mail(
    Extension(mail_service): Extension<Arc<MailService>>,
    Path(mail_id): Path<String>,
    Json(payload): Json<UpdateMailRequest>,
) -> Result<Json<UpdateMailResponse>, AppError> {
    // Same reasoning as add_mail: a caller must not be able to move an already-sent mail
    // back into the queue and have it delivered again.
    if let Some(status) = payload.status.as_deref() {
        if !is_recordable_status(status) {
            return Err(AppError::BadRequestError(Some(format!(
                "status must be one of {}",
                RECORDABLE_STATUSES.join(", ")
            ))));
        }
    }

    let updated_mail = mail_service.update_mail(mail_id, payload).await?;

    Ok(Json(updated_mail))
}

#[utoipa::path(
    delete,
    path = "/api/mails/{mail_id}",
    responses(
        (status = 200, description = "Delete a mail", body = DeleteMailResponse),
        (status = 404)
    )
)]
pub async fn delete_mail(
    Extension(mail_service): Extension<Arc<MailService>>,
    Path(mail_id): Path<String>,
) -> Result<Json<DeleteMailResponse>, AppError> {
    let deleted_mail = mail_service.delete_mail(mail_id).await?;

    Ok(Json(deleted_mail))
}

#[utoipa::path(
    get,
    path = "/api/mails/bounce",
    responses(
        (status = 200, description = "Get all bounced mails", body = Vec<GetMailResponse>),
        (status = 404)
    )
)]
pub async fn get_bounced_mails(
    Extension(mail_service): Extension<Arc<MailService>>,
) -> Result<Json<Vec<GetMailResponse>>, AppError> {
    let all_bounced_mails = mail_service.fetch_bounced_mails().await?;

    let mut responses = Vec::new();

    if all_bounced_mails.is_empty() {
        return Ok(Json(vec![]));
    }
    all_bounced_mails.iter().for_each(|mail| {
        responses.push(GetMailResponse {
            id: mail.id.clone(),
            mail_message: mail.mail_message.clone(),
            email: mail.email.clone(),
            template_id: mail.template_id,
            campaign_id: mail.campaign_id,
            sent_at: mail.sent_at,
            open: mail.open,
            clicks: mail.clicks,
            status: mail.status.clone(),
            status_reason: mail.reason.clone(),
            server_id: mail.server_id,
            scheduled_at: mail.scheduled_at,
            attempts: mail.attempts,
            last_error: mail.last_error.clone(),
            from_name: mail.from_name.clone(),
        });
    });
    Ok(Json(responses))
}
