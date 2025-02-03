use axum::{
    routing::{get, post, patch, delete},
    Router
};

use crate::handlers::campaign_sender::{
    create_campaign_sender,
    get_campaign_senders,
    update_campaign_sender,
    delete_campaign_sender,
    get_campaign_sender_by_id
};

pub fn campaign_sender_routes() -> Router {
    Router::new()
        .route("/", get(get_campaign_senders))
        .route("/", post(create_campaign_sender))
        .route("/{senderId}", get(get_campaign_sender_by_id))
        .route("/{senderId}", patch(update_campaign_sender))
        .route("/{senderId}", delete(delete_campaign_sender))
}