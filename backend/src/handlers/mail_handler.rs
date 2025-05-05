use crate::models::mail::{
    CreateMailRequest, CreateMailResponse, DeleteMailResponse, GetMailResponse, MailQuery, UpdateMailRequest, UpdateMailResponse
};
use crate::services::mail_service::{MailService, MailServiceTrait};

use std::sync::Arc;
use axum::{
    extract:: { Extension, Path, Query }, Json
};
use crate::error::AppError;

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
    Json(payload): Json<CreateMailRequest>,
) -> Result<Json<Vec<CreateMailResponse>>, AppError> {
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
    responses(
        (status = 200, description = "Get all mails", body = Vec<GetMailResponse>),
        (status = 404)
    )
)]
pub async fn get_all_mails(
    Extension(mail_service): Extension<Arc<MailService>>,
    query: Query<MailQuery>
) -> Result<Json<Vec<GetMailResponse>>, AppError> {
    let all_mails = mail_service.get_all_mails(
        query.campaign_ids,
        query.from,
        query.to
    ).await?;

    let mut responses = Vec::new();

    if all_mails.is_empty() {
        return Ok(Json(vec![]));
    }
    all_mails.iter().for_each(|mail| {
        responses.push(GetMailResponse {
            id: mail.id.clone(),
            mail_message: mail.mail_message.clone(),
            email: mail.email.clone(),
            template_id: mail.template_id.clone(),
            campaign_id: mail.campaign_id,
            sent_at: mail.sent_at,
            open: mail.open,
            clicks: mail.clicks,
            status: mail.status.clone(),
            status_reason: mail.reason.clone(),
            server_id: mail.server_id.clone(),
            scheduled_at: mail.scheduled_at,
            attempts: mail.attempts,
            last_error: mail.last_error.clone(),
        });
    });
    Ok(Json(responses))

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