use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use crate::handlers::campaign::{create_campaign, delete_campaign, get_all_campaigns, get_campaign_by_id, update_campaign, send_campaign_email};

pub fn campaign_routes() -> Router {
    Router::new()
        .route("/", post(create_campaign))
        .route("/", get(get_all_campaigns))
        .route("/{campaign_id}", get(get_campaign_by_id))
        .route("/{campaign_id}", patch(update_campaign))
        .route("/{campaign_id}", delete(delete_campaign))
        .route("/{campaign_id}/send", post(send_campaign_email))
}