use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use crate::handlers::template::{
    delete_template, get_templates, get_templates_by_id, create_template, update_template, send_templated_email
};


pub fn template_routes() -> Router {
    Router::new()
        .route("/{templateId}", get(get_templates_by_id))
        .route("/", get(get_templates))
        .route("/", post(create_template))
        .route("/{templateId}", patch(update_template))
        .route("/{templateId}", delete(delete_template))
        .route("/{templateId}/send", post(send_templated_email))
}