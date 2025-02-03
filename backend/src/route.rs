use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{
    template as template,
    contact as contact,
    campaign as campaign,
    campaign_sender as campaign_sender
};

use crate::handlers::list::{create_list, get_lists, update_list, get_list_by_id, delete_list, add_contacts_to_list, remove_contacts_from_list};

use crate::routes::list::list_routes;
use crate::routes::{
    template as template_routes,
    contact as contact_routes,
    campaign as campaign_routes,
    campaign_senders as campaign_senders_routes
};
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
        campaign::get_all_campaigns, 
        campaign::get_campaign_by_id, 
        campaign::create_campaign, 
        campaign::update_campaign, 
        campaign::delete_campaign, 
        campaign::send_campaign_email,
        campaign_sender::create_campaign_sender,
        campaign_sender::get_campaign_senders,
        campaign_sender::get_campaign_sender_by_id,
        campaign_sender::update_campaign_sender,
        campaign_sender::delete_campaign_sender,
    ),
    servers(
        (url = "/", description = "Default server")
    )
)]
pub struct ApiDoc;

pub fn create_router() -> Router {
    let api_routes = Router::new()
        .nest("/templates", template_routes::template_routes())
        .nest("/list", list_routes())
        .nest("/contacts", contact_routes::contact_routes())
        .nest("/campaigns", campaign_routes::campaign_routes())
        .nest("/campaign-senders", campaign_senders_routes::campaign_sender_routes());

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/mail-service", ApiDoc::openapi())
        )
}
