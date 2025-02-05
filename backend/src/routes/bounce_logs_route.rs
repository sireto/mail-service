use axum::{
    routing::{
        get,
        post,
        delete
    }, Router };

use crate::handlers::bounce_logs_handler::{
    delete_bounce, get_all_bounces, get_bounces_by_contact_id, handle_sns_notification
};

pub fn bounce_logs_routes() -> Router {
    Router::new()
        .route("/sns/bounce", post(handle_sns_notification))
        .route("/", get(get_all_bounces))
        .route("/contacts/{contactId}", get(get_bounces_by_contact_id))
        .route("/{bounceId}", delete(delete_bounce))
}