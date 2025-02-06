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

use crate::servers::servers_handler::{
    create_server, 
    get_servers, 
    get_server_by_id, 
    delete_server,
    update_server
};

use crate::servers::servers_services;
use crate::servers::servers_repo;

pub fn servers_routes()-> Router<> {

    let server_repo = Arc::new(servers_repo::ServerRepoImpl);
    let server_service = servers_services::ServerService::new(server_repo);

    Router::new()
        .route("/", post(create_server))
        .route("/", get(get_servers))
        .route("/{serverId}", get(get_server_by_id))
        .route("/{serverId}", patch(update_server))
        .route("/{serverId}", delete(delete_server))
        .layer(Extension(Arc::new(server_service)))
}