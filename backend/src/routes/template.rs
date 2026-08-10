use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Extension, Router,
};

use crate::handlers::template::{
    create_template, delete_template, get_templates, get_templates_by_id, parse_mjml_to_html, send_templated_email,
    update_template,
};

use crate::repositories::mail_repository;
use crate::services::mail_service;

pub fn template_routes() -> Router {
    let mail_repo = Arc::new(mail_repository::MailRepositoryImpl);
    let mail_service = mail_service::MailService::new(mail_repo);

    Router::new()
        .route("/{templateId}", get(get_templates_by_id))
        .route("/", get(get_templates))
        .route("/", post(create_template))
        .route("/{templateId}", patch(update_template))
        .route("/{templateId}", delete(delete_template))
        .route("/{templateId}/send", post(send_templated_email))
        .route("/parse-mjml", post(parse_mjml_to_html))
        .layer(Extension(Arc::new(mail_service)))
}
