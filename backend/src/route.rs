use axum::{
    routing::{
        get,
        post,
        patch,
        delete
    }, Router };

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::template::{
    create_template, delete_template, get_templates, get_templates_by_id, send_templated_email, update_template
};

use crate::handlers::list::{create_list, get_lists, update_list, get_list_by_id, delete_list, add_contacts_to_list, remove_contacts_from_list};


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
        list::delete_list, 
        list::add_contacts_to_list, 
        list::remove_contacts_from_list
    ),
    servers(
        (url = "/", description = "Default server")
    )
)]


struct ApiDoc;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/namespaces/{namespace_id}/templates/{templateId}", get(get_templates_by_id))
        .route("/api/namespaces/{namespace_id}/templates", get(get_templates))
        .route("/api/templates", post(create_template))
        .route("/api/namespaces/{namespace_id}/templates/{templateId}", patch(update_template))
        .route("/api/namespaces/{namespace_id}/templates/{templateId}", delete(delete_template))
        .route("/api/namespaces/{namespace_id}/templates/{templateId}/send", post(send_templated_email))
        .route("/api/list", post(create_list))
        .route("/api/namespaces/{namespace_id}/list", get(get_lists))
        .route("/api/namespaces/{namespace_id}/list/{list_id}", get(get_list_by_id))
        .route("/api/namespaces/{namespace_id}/list/{list_id}", patch(update_list))
        .route("/api/namespaces/{namespace_id}/list/{list_id}", delete(delete_list))
        .route("/api/list/addContacts/{list_id}", post(add_contacts_to_list))
        .route("/api/list/removeContacts/{list_id}", delete(remove_contacts_from_list))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/mail-service", ApiDoc::openapi())
        )
}