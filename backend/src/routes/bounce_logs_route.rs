use std::sync::Arc;
use axum::{
    routing::{
        get,
        post,
        delete
    }, Router, Extension };

use crate::handlers::bounce_logs_handler::{
    delete_bounce, get_all_bounces, get_bounces_by_contact_id, handle_sns_notification
};

use crate::services::mail_service;
use crate::repositories::mail_repository;


pub fn bounce_logs_routes() -> Router {
    let mail_repo = Arc::new(mail_repository::MailRepositoryImpl);
    let mail_service = mail_service::MailService::new(mail_repo);

    Router::new()
        .route("/sns/bounce", post(handle_sns_notification))
        .route("/", get(get_all_bounces))
        .route("/contacts/{contactId}", get(get_bounces_by_contact_id))
        .route("/{bounceId}", delete(delete_bounce))
        .layer(Extension(Arc::new(mail_service)))
}