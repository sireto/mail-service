use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{appState::AppState, handlers::{ template::get_templates_by_id}};

use crate::handlers::template::{
    delete_template, get_templates, create_template, update_template, send_templated_email
};

use crate::handlers::list::{create_list, get_lists, update_list, get_list_by_id};


use crate::handlers::template as template;
use crate::handlers::list as list;

#[derive(OpenApi)]
#[openapi(
    paths(
        template::get_templates,
        template::get_templates_by_id,
        template::create_template,
        template::update_template,
        template::delete_template,
        template::send_templated_email, 
        list::create_list, 
        list::get_lists, 
        list::get_list_by_id, 
        list::update_list, 
        list::delete_list
    ),
    servers(
        (url = "/", description = "Default server")
    )
)]


struct ApiDoc;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/templates/{templateId}", get(get_templates_by_id))
        .route("/api/templates", get(get_templates))
        .route("/api/templates", post(create_template))
        .route("/api/templates/{templateId}", patch(update_template))
        .route("/api/templates/{templateId}", delete(delete_template))
        .route("/api/templates/{templateId}/send", post(send_templated_email))
        .route("/api/list", post(create_list))
        .route("/api/list", get(get_lists))
        .route("/api/list/{list_id}", get(get_list_by_id))
        .route("/api/list/{list_id}", patch(update_list))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/mail-service", ApiDoc::openapi())
        )
}