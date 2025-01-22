use std::sync::Arc;

use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };


use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{appState::AppState};

use crate::handlers::template::{
    create_template, get_templates, get_template_by_id, health_checker
};

use crate::handlers::template as template;

#[derive(OpenApi)]
#[openapi(
    paths(
        template::get_templates,
        template::create_template,
    ),
    tags (
        (name="Templates", description="Operations about templates")
    )
)]
struct ApiDoc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/test", get(health_checker))
        .route("/api/templates/{id}", get(get_template_by_id))
        .route("/api/templates", get(get_templates))
        .route("/api/templates", post(create_template))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/mail-service", ApiDoc::openapi())
        )
        .with_state(state)
}