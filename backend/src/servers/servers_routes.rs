use std::sync::Arc;

use axum::Extension;
use::axum::{
    routing::{
        get, 
        post, 
        patch, 
        delete
    }, Router
};

use crate::{repositories::mail_repository, servers::servers_handler::{
    check_credentials, create_server, delete_server, get_mails_from_server, get_server_by_id, get_servers, send_mail_from_server, update_server
}, services::mail_service};

use crate::servers::servers_services;
use crate::servers::servers_repo;
use crate::repositories::template_repo;
use crate::services::template_service;

pub fn servers_routes()-> Router<> {

    let server_repo = Arc::new(servers_repo::ServerRepoImpl);
    let server_service = servers_services::ServerService::new(server_repo);

    let template_repo = Arc::new(template_repo::TemplateRespositoryImpl);
    let template_service = template_service::TemplateService::new(template_repo);

    Router::new()
        .route("/", post(create_server))
        .route("/", get(get_servers))
        .route("/{serverId}", get(get_server_by_id))
        .route("/{serverId}", patch(update_server))
        .route("/{serverId}", delete(delete_server))
        .route("/check-smtp", post(check_credentials))
        .route("/{serverId}/send", post(send_mail_from_server))
        .route("/{serverId}/mails", get(get_mails_from_server))
        .layer(Extension(Arc::new(server_service)))
        .layer(Extension(Arc::new(template_service)))
}