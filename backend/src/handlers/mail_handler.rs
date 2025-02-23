use crate::models::mail::{
    CreateMailRequest, CreateMailResponse, DeleteMailResponse, GetMailResponse, MailQuery, MailWithDetails, UpdateMailRequest, UpdateMailResponse
};
use crate::services::mail_service as mail_service;
use crate::services::contact as contact_service;

use axum::{
    extract:: { Path, Query }, Json, http::StatusCode
};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/mails",
    responses(
        (status = 200, description = "Add a new mail in record", body = Vec<CreateMailRequest>),
        (status = 404)
    )
)]
pub async fn add_mail(
    Json(payload): Json<CreateMailRequest>,
) -> Result<Json<Vec<CreateMailResponse>>, (StatusCode, String)> {
    let created_mail = mail_service::create_mail(payload).await?;

    Ok(Json(created_mail))
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
    query: Query<MailQuery>
) -> Result<Json<Vec<GetMailResponse>>, (StatusCode, String)> {
    let all_mails = mail_service::get_all_mails(
        query.campaign_ids,
        query.from,
        query.to
    ).await?;

    let mut responses = Vec::new();

    all_mails.iter().for_each(|mail| {
        responses.push(GetMailResponse {
            id: mail.id.clone(),
            mail_message: mail.mail_message.clone(),
            email: mail.email.clone(),
            template_id: mail.template_id.clone(),
            campaign_id: mail.campaign_id,
            sent_at: mail.sent_at,
            status: mail.status.clone(),
            status_reason: mail.reason.clone(),
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
    Path(mail_id): Path<String>,
    Json(payload): Json<UpdateMailRequest>,
) -> Result<Json<UpdateMailResponse>, (StatusCode, String)> {
    let updated_mail = mail_service::update_mail(mail_id, payload).await?;

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
    Path(mail_id): Path<String>,
) -> Result<Json<DeleteMailResponse>, (StatusCode, String)> {
    // let uuid_id = Uuid::parse_str(&mail_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid contact ID format".to_string()))?;

    let deleted_mail = mail_service::delete_mail(mail_id).await?;

    Ok(Json(deleted_mail))
}