use crate::models::mail::{
    CreateMailRequest, 
    CreateMailResponse, 
    GetMailResponse, 
    UpdateMailRequest,
    UpdateMailResponse,
    DeleteMailResponse
};
use crate::services::mail as mail_service;
use axum::{
    extract:: Path, Json, http::StatusCode
};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/mails",
    responses(
        (status = 200, description = "Add a new mail in record", body = CreateMailRequest),
        (status = 404)
    )
)]
pub async fn add_mail(
    Json(payload): Json<CreateMailRequest>,
) -> Result<Json<CreateMailResponse>, (StatusCode, String)> {
    let created_mail = mail_service::create_mail(payload).await?;

    Ok(Json(created_mail))
}

#[utoipa::path(
    get,
    path = "/api/mails",
    responses(
        (status = 200, description = "Get all mails", body = Vec<GetMailResponse>),
        (status = 404)
    )
)]
pub async fn get_all_mails() -> Result<Json<Vec<GetMailResponse>>, (StatusCode, String)> {
    let all_mails = mail_service::get_all_mails().await?;

    Ok(Json(all_mails))
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
    let uuid_id = Uuid::parse_str(&mail_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid contact ID format".to_string()))?;

    let updated_mail = mail_service::update_mail(uuid_id, payload).await?;

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
    let uuid_id = Uuid::parse_str(&mail_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid contact ID format".to_string()))?;

    let deleted_mail = mail_service::delete_mail(uuid_id).await?;

    Ok(Json(deleted_mail))
}