use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use crate::handlers::mail::{
    add_mail,
    get_all_mails,
    update_mail,
    delete_mail
};

pub fn mail_routes() -> Router {
    Router::new()
        .route("/", post(add_mail))
        .route("/", get(get_all_mails))
        .route("/{mailId}", patch(update_mail))
        .route("/{mailId}", delete(delete_mail))
}