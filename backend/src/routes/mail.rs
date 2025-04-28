use std::sync::Arc;

use axum::{
    routing::{
        delete, get, patch, post
    }, Extension, Router };

use crate::handlers::mail_handler::{
    add_mail, delete_mail, get_all_mails, update_mail
};

use crate::services::mail_service as service;
use crate::repositories::mail_repository;

pub fn mail_routes() -> Router {
    let mail_repo = Arc::new(mail_repository::MailRepositoryImpl);
    let mail_service = service::MailService::new(mail_repo);

    Router::new()
        .route("/", post(add_mail))
        .route("/", get(get_all_mails))
        .route("/{mailId}", patch(update_mail))
        .route("/{mailId}", delete(delete_mail))
        .layer(Extension(Arc::new(mail_service)))
}