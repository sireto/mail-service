use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post}, Router
};

use crate::{handlers::campaign_sender::{
    create_campaign_sender, delete_campaign_sender, get_campaign_sender_by_id, get_campaign_senders, get_verified_identities, update_campaign_sender, validate_email_identity, send_test_email
}, servers::{servers_repo, servers_services}};


pub fn campaign_sender_routes() -> Router {

    Router::new()
        .route("/", get(get_campaign_senders))
        .route("/", post(create_campaign_sender))
        .route("/{senderId}", get(get_campaign_sender_by_id))
        .route("/{senderId}", patch(update_campaign_sender))
        .route("/{senderId}", delete(delete_campaign_sender))
        .route("/email-identities/validate", post(validate_email_identity))
        .route("/email-identities", get(get_verified_identities))
        .route("/test-email", post(send_test_email))
}