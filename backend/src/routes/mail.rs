use axum::{
    routing::{
        delete, get, patch, post
    }, Router };

use crate::handlers::mail_handler::{
    add_mail, delete_mail, get_all_mails, update_mail
};

pub fn mail_routes() -> Router {
    Router::new()
        .route("/", post(add_mail))
        .route("/", get(get_all_mails))
        .route("/{mailId}", patch(update_mail))
        .route("/{mailId}", delete(delete_mail))
}