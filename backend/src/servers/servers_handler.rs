use crate::{error::AppError, servers::servers_model::{SendMailFromServerRequest, SendMailFromServerResponse, ServerRequest, ServerResponse}};
use axum::{
    extract::Extension, response::IntoResponse, Json, http::StatusCode
};
use utoipa::ToSchema;
use crate::models::mail::GetMailResponse;

use std::sync::Arc;
use uuid::Uuid;

use crate::servers::servers_services::{ServerServiceTrait, ServerService};

use serde::Serialize;

#[derive(Serialize ,ToSchema)]
struct SmtpCheckResponse {
    success: bool,
    message: String,
}

#[utoipa::path(
    post, 
    path="/api/servers", 
    responses(
        (status=200, description = "Create a contact", body= ServerRequest), 
        (status=404)
    )
)]
pub async fn create_server(Extension(server_service): Extension<Arc<ServerService>>, Json(payload): Json<ServerRequest>)->Result<Json<ServerResponse>, AppError> {
    let created_server = server_service.create_server(payload).await?;

    let server_response: ServerResponse = created_server.into();

    Ok(Json(server_response))
}

#[utoipa::path(
    get, 
    path="/api/servers", 
    responses(
        (status=200, description = "Get all servers", body = ServerResponse),
        (status=400)
    )
)]
pub async fn get_servers(
    Extension(server_service): Extension<Arc<ServerService>>
) -> Result<Json<Vec<ServerResponse>>, AppError> {
    let servers = server_service.get_all_servers().await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;
    
    let response: Vec<ServerResponse> = servers.into_iter().map(|server| {
        let server_response: ServerResponse = server.into();

        return server_response;
    }).collect(); 

    Ok(Json(response))
}

#[utoipa::path(
    get, 
    path="/api/servers/{server_id}", 
    responses(
        (status=200, description = "Get server by ID", body = ServerResponse),
        (status=400, description = "Invalid server ID format"),
        (status=404, description = "Server not found")
    )
)]
pub async fn get_server_by_id(
    Extension(server_service): Extension<Arc<ServerService>>,
    server_id: axum::extract::Path<String>, 
) -> Result<Json<ServerResponse>, AppError> {
    let server = server_service.get_server_by_id(&server_id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let server_response: ServerResponse = server.into();

    Ok(Json(server_response))
}

#[utoipa::path(
    patch, 
    path="/api/servers/{server_id}", 
    request_body = ServerRequest,
    responses(
        (status=200, description = "Update server", body = ServerResponse),
        (status=400, description = "Invalid server ID format"),
        (status=404, description = "Server not found"),
        (status=500, description = "Internal server error")
    )
)]
pub async fn update_server(
    Extension(server_service): Extension<Arc<ServerService>>,
    server_id: axum::extract::Path<String>, 
    Json(payload): Json<ServerRequest>
) -> Result<Json<ServerResponse>, AppError> {
    
    let updated_server = server_service.update_server(&server_id, payload).await?;

    let server_response: ServerResponse = updated_server.into();

    Ok(Json(server_response))
}

#[utoipa::path(
    delete, 
    path="/api/servers/{server_id}", 
    responses(
        (status=204, description = "Delete server"),
        (status=400, description = "Invalid server ID format"),
        (status=404, description = "Server not found")
    )
)]
pub async fn delete_server(
    Extension(server_service): Extension<Arc<ServerService>>,
    server_id: axum::extract::Path<String>, 
) -> Result<StatusCode, AppError> {
    server_service.delete_server(&server_id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post, 
    path="/api/servers/check-smtp", 
    responses(
        (status=200, description = "Check SMTP Credentials", body= SmtpCheckResponse), 
        (status=404)
    )
)]
pub async fn check_credentials(
    Extension(server_service): Extension<Arc<ServerService>>,
    Json(payload): Json<ServerRequest>
) -> Result<impl IntoResponse, AppError> {
    server_service.check_credentials(payload).await?;

    let response = SmtpCheckResponse {
        success: true,
        message: "SMTP credentials are valid".to_string(),
    };

    Ok((StatusCode::OK, axum::Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/servers/{serverId}/send",
    responses(
        (status = 200, description = "Mail send successfully", body = SendMailFromServerResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn send_mail_from_server(
    Extension(server_service): Extension<Arc<ServerService>>,
    server_id: axum::extract::Path<String>,
    Json(payload): Json<SendMailFromServerRequest> 
) -> Result<Json<SendMailFromServerResponse>, AppError> {
    let template_id = payload.template_id;
    let mail = server_service.send_mail_from_server(&server_id, template_id, payload.receiver.unwrap()).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    Ok(Json(SendMailFromServerResponse {
        id: mail.id,
        template_id,
        server_id: mail.server_id,
        mail_send_ids: mail.mail_send_ids,
        sent_at: mail.sent_at,
        status: mail.status,
    }))
}

#[utoipa::path(
    get,
    path = "/api/servers/{serverId}/mails",
    responses(
        (status = 200, description = "Mail send successfully", body = GetMailResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Template not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_mails_from_server(
    Extension(server_service): Extension<Arc<ServerService>>,
    server_id: axum::extract::Path<String>,
) -> Result<Json<Vec<GetMailResponse>>, AppError> {
    let server_uuid = Uuid::parse_str(&server_id).map_err(|_| AppError::BadRequestError(Some("Invalid server ID format".to_string())))?;

    let mails = server_service.get_mails_by_server_id(server_uuid).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;
    let mails_response: Vec<GetMailResponse> = mails.into_iter().map(|mail| GetMailResponse {
        id: mail.id,
        template_id: mail.template_id,
        server_id: mail.server_id,
        sent_at: mail.sent_at,
        status: mail.status,
        open: mail.open,
        clicks: mail.clicks,
        email: mail.email,
        status_reason: mail.reason,
        mail_message: mail.mail_message,
        campaign_id: mail.campaign_id,
        scheduled_at: mail.scheduled_at,
        attempts: mail.attempts,
        last_error: mail.last_error,
    }).collect();

    Ok(Json(mails_response))
}