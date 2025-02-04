use std::sync::Arc;

use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use crate::{handlers::namespace::{
    create_namespace,
    get_all_namespaces,
    get_namespace_by_id,
    update_namespace,
    delete_namespace
}, services::namespace::NamespaceService};

pub fn namepsace_routes(service: NamespaceService) -> Router {
    Router::new()
        .route("/", post(create_namespace))
        .route("/", get(get_all_namespaces))
        .route("/{namespaceId}", get(get_namespace_by_id))
        .route("/{namespaceId}", patch(update_namespace))
        .route("/{namespaceId}", delete(delete_namespace))
        .with_state(Arc::new(service))
}