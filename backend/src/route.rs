use std::sync::Arc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{
    template,
    contact,
    mail,
    namespace,
};

use crate::handlers::list::{create_list, get_lists, update_list, get_list_by_id, delete_list, add_contacts_to_list, remove_contacts_from_list};

use crate::routes::list::list_routes;
use crate::routes::{
    template as template_routes,
    contact as contact_routes,
    mail as mail_routes,
    namespace as namespace_routes,
};
use crate::handlers::list as list;

use crate::services::namespace as namespace_service;
use crate::repositories::namespace::NamespaceRepositoryImpl;


#[derive(OpenApi)]
#[openapi(
    paths(
        template::get_templates,
        template::get_templates_by_id,
        template::create_template,
        template::update_template,
        template::delete_template,
        template::send_templated_email,
        contact::create_contact,
        contact::get_contacts,
        contact::update_contact,
        contact::delete_contact,
        contact::get_contact_by_id, 
        list::create_list, 
        list::get_lists, 
        list::get_list_by_id, 
        list::update_list, 
        list::delete_list, 
        list::add_contacts_to_list, 
        list::remove_contacts_from_list,
        mail::add_mail,
        mail::get_all_mails,
        mail::update_mail,
        mail::delete_mail,
        namespace::create_namespace,
        namespace::get_all_namespaces,
        namespace::get_namespace_by_id,
        namespace::update_namespace,
        namespace::delete_namespace
    ),
    servers(
        (url = "/", description = "Default server")
    )
)]
pub struct ApiDoc;

pub fn create_router() -> Router {
    let namespace_repo = Arc::new(NamespaceRepositoryImpl);
    let namespace_service = namespace_service::NamespaceService::new(namespace_repo);

    let api_routes = Router::new()
        .nest("/templates", template_routes::template_routes())
        .nest("/list", list_routes())
        .nest("/contacts", contact_routes::contact_routes())
        .nest("/mails", mail_routes::mail_routes())
        .nest("/namespaces", namespace_routes::namepsace_routes(namespace_service));

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/mail-service", ApiDoc::openapi())
        )
}
