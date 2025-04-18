use crate::{error::AppError, servers::servers_model::{SendMailFromServerRequest, SendMailFromServerResponse, ServerRequest, ServerResponse}};
use axum::{
    extract::Extension, response::IntoResponse, Json, http::StatusCode
};
use utoipa::ToSchema;
use crate::models::mail::GetMailResponse;

use std::sync::Arc;
use uuid::Uuid;

use crate::servers::servers_services::{ServerServiceTrait, ServerService};
use crate::utils::server_utils::secure_server_response;


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

    let safe_aws = created_server.aws_credentials.map(|creds| secure_server_response(creds));

    Ok(Json(ServerResponse {
        id: created_server.id,
        active: created_server.active,
        host: created_server.host,
        smtp_username: created_server.smtp_username,
        smtp_password: "*************".to_string(),
        namespace_id: created_server.namespace_id,
        tls_type: created_server.tls_type,
        server_type: created_server.server_type,
        aws_credentials: safe_aws,
        port: created_server.port,
        created_at: created_server.created_at, 
        updated_at: created_server.updated_at,
        default_from_email: created_server.default_from_email,
    }))
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
        let safe_aws = server.aws_credentials.map(|creds| secure_server_response(creds));

        return ServerResponse {
          id: server.id,
          active: server.active,
          host: server.host,
          smtp_username: server.smtp_username,
          smtp_password: "*************".to_string(),
          namespace_id: server.namespace_id,
          tls_type: server.tls_type,
          server_type: server.server_type, 
          port: server.port,
          aws_credentials: safe_aws,
          created_at: server.created_at,
          updated_at: server.updated_at,
          default_from_email: server.default_from_email,
          };  
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

    let safe_aws = server.aws_credentials.map(|creds| secure_server_response(creds));

    Ok(Json(ServerResponse {
        id: server.id,
        active: server.active,
        host: server.host,
        smtp_username: server.smtp_username,
        smtp_password: "*************".to_string(),
        namespace_id: server.namespace_id,
        tls_type: server.tls_type,
        server_type: server.server_type, 
        aws_credentials: safe_aws,
        port: server.port,
        created_at: server.created_at,
        updated_at: server.updated_at,
        default_from_email: server.default_from_email,
    }))
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

    let safe_aws = updated_server.aws_credentials.map(|creds| secure_server_response(creds));


    Ok(Json(ServerResponse {
        id: updated_server.id,
        active: updated_server.active,
        host: updated_server.host,
        smtp_username: updated_server.smtp_username,
        smtp_password: "*************".to_string(),
        namespace_id: updated_server.namespace_id,
        tls_type: updated_server.tls_type,
        server_type: updated_server.server_type,
        aws_credentials: safe_aws,
        port: updated_server.port,
        created_at: updated_server.created_at,
        updated_at: updated_server.updated_at,
        default_from_email: updated_server.default_from_email,
    }))
}

#[utoipa::path(
    delete, 
    path="/api/servers/{server_id}", 
    responses(
        (status=200, description = "Delete server", body = ServerResponse),
        (status=400, description = "Invalid server ID format"),
        (status=404, description = "Server not found")
    )
)]
pub async fn delete_server(
    Extension(server_service): Extension<Arc<ServerService>>,
    server_id: axum::extract::Path<String>, 
) -> Result<Json<ServerResponse>, AppError> {
    let deleted_server = server_service.delete_server(&server_id).await.map_err(|err| AppError::NotFoundError(Some(err.to_string())))?;

    let safe_aws = deleted_server.aws_credentials.map(|creds| secure_server_response(creds));

    Ok(Json(ServerResponse {
        id: deleted_server.id,
        active: deleted_server.active,
        host: deleted_server.host,
        smtp_username: deleted_server.smtp_username,
        smtp_password: "*************".to_string(),
        namespace_id: deleted_server.namespace_id,
        tls_type: deleted_server.tls_type,
        server_type: deleted_server.server_type, 
        aws_credentials: safe_aws,
        port: deleted_server.port,
        created_at: deleted_server.created_at,
        updated_at: deleted_server.updated_at,
        default_from_email: deleted_server.default_from_email,
    }))
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
    }).collect();

    Ok(Json(mails_response))
}