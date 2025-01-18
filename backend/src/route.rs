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

use crate::appState::AppState;

use crate::handlers::template::{
    create_template, delete_template, get_templates, update_template
};

use crate::handlers::template as template;

#[derive(OpenApi)]
#[openapi(
    paths(
        template::get_templates,
        template::create_template,
        template::update_template,
        template::delete_template
    ),
    tags (
        (name="Templates", description="Operations about templates")
    )
)]
struct ApiDoc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/templates", get(get_templates))
        .route("/api/templates", post(create_template))
        .route("/api/templates/{templateId}", patch(update_template))
        .route("/api/templates/{templateId}", delete(delete_template))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/mail-service", ApiDoc::openapi())
        )
        .with_state(state)
}