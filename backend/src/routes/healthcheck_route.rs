use axum::{
    routing::get, Router };

use crate::handlers::healthcheck_handler::check_health;

pub fn healthcheck_routes() -> Router {
    Router::new()
        .route("/", get(check_health))
}