use crate::servers::servers_model::{ServerRequest, ServerResponse, DeleteServerResponse, Server};
use axum::{
    extract::Extension, http::StatusCode, Json
};

use std::sync::Arc;

use super::servers_services::ServerServiceTrait;
use crate::servers::servers_services::ServerService;

#[utoipa::path(
    post, 
    path="/api/servers", 
    responses(
        (status=200, description = "Create a contact", body= ServerRequest), 
        (status=404)
    )
)]
pub async fn create_server(Extension(server_service): Extension<Arc<ServerService>>, Json(payload): Json<ServerRequest>)->Result<Json<ServerResponse>, (StatusCode, String)> {
    let created_server = server_service.create_server(payload).await?;
    Ok(Json(ServerResponse {
        id: created_server.id,
        host: created_server.host,
        smtp_username: created_server.smtp_username,
        smtp_password: created_server.smtp_password,
        namespace_id: created_server.namespace_id,
        tls_type: created_server.tls_type,
        port: created_server.port,
        created_at: created_server.created_at, 
        updated_at: created_server.updated_at
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
) -> Result<Json<Vec<ServerResponse>>, (StatusCode, String)> {
    
    let servers = server_service.get_all_servers().await?;
    let response: Vec<ServerResponse> = servers.into_iter().map(|server| ServerResponse {
        id: server.id,
        host: server.host,
        smtp_username: server.smtp_username,
        smtp_password: server.smtp_password,
        namespace_id: server.namespace_id,
        tls_type: server.tls_type,
        port: server.port,
        created_at: server.created_at,
        updated_at: server.updated_at,
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
) -> Result<Json<ServerResponse>, (StatusCode, String)> {
    let server = server_service.get_server_by_id(&server_id).await?;
    Ok(Json(ServerResponse {
        id: server.id,
        host: server.host,
        smtp_username: server.smtp_username,
        smtp_password: server.smtp_password,
        namespace_id: server.namespace_id,
        tls_type: server.tls_type,
        port: server.port,
        created_at: server.created_at,
        updated_at: server.updated_at,
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
) -> Result<Json<ServerResponse>, (StatusCode, String)> {
    
    let updated_server = server_service.update_server(&server_id, payload).await?;

    Ok(Json(ServerResponse {
        id: updated_server.id,
        host: updated_server.host,
        smtp_username: updated_server.smtp_username,
        smtp_password: updated_server.smtp_password,
        namespace_id: updated_server.namespace_id,
        tls_type: updated_server.tls_type,
        port: updated_server.port,
        created_at: updated_server.created_at,
        updated_at: updated_server.updated_at,
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
) -> Result<Json<ServerResponse>, (StatusCode, String)> {
    let deleted_server = server_service.delete_server(&server_id).await?;
    Ok(Json(ServerResponse {
        id: deleted_server.id,
        host: deleted_server.host,
        smtp_username: deleted_server.smtp_username,
        smtp_password: deleted_server.smtp_password,
        namespace_id: deleted_server.namespace_id,
        tls_type: deleted_server.tls_type,
        port: deleted_server.port,
        created_at: deleted_server.created_at,
        updated_at: deleted_server.updated_at,
    }))
}
