use axum::{routing::{
    get,
    post,
    patch,
    delete
}, Router};

use crate::handler::{
    get_templates,
    create_template,
    update_template,
    delete_template
};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/templates", get(get_templates))
        .route("/api/templates", post(create_template))
        .route("/api/templates/:templateId", patch(update_template))
        .route("/api/templates/:templateId", delete(delete_template))
}