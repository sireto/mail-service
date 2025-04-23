use axum::Router;
use axum::routing::get;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{
    template,
    contact,
    mail_handler,
    campaign as campaign,
    campaign_sender as campaign_sender,
    bounce_logs_handler,
    healthcheck_handler,
};
use crate::routes::healthcheck_route::healthcheck_routes;
use crate::servers::servers_handler as servers;

use crate::routes::list::list_routes;
use crate::routes::{
    template as template_routes,
    contact as contact_routes,
    mail as mail_routes,
    campaign as campaign_routes,
    campaign_senders as campaign_senders_routes,
    bounce_logs_route,
};
use crate::handlers::list as list;
use crate::servers::servers_routes::servers_routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        template::get_templates,
        template::get_templates_by_id,
        template::create_template,
        template::update_template,
        template::delete_template,
        template::send_templated_email,
        template::parse_mjml_to_html,
        contact::create_contacts,
        contact::get_contacts,
        contact::update_contact,
        contact::delete_contact,
        contact::get_contact_by_id, 
        contact::check_email,
        contact::import_contacts,
        list::create_list, 
        list::get_lists, 
        list::get_list_by_id, 
        list::update_list, 
        list::delete_list, 
        list::add_contacts_to_list, 
        list::remove_contacts_from_list, 
        list::get_contacts_from_lists,
        campaign::get_all_campaigns, 
        campaign::get_campaign_by_id, 
        campaign::create_campaign, 
        campaign::update_campaign, 
        campaign::delete_campaign, 
        campaign::send_campaign_email,
        campaign::send_campaign_email_smtp,
        campaign_sender::create_campaign_sender,
        campaign_sender::get_campaign_senders,
        campaign_sender::get_campaign_sender_by_id,
        campaign_sender::update_campaign_sender,
        campaign_sender::delete_campaign_sender,
        campaign_sender::validate_email_identity, 
        campaign_sender::get_verified_identities,
        campaign_sender::send_test_email,
        servers::create_server,
        servers::get_servers, 
        servers::get_server_by_id, 
        servers::update_server, 
        servers::delete_server,
        servers::check_credentials,
        servers::send_mail_from_server,
        servers::get_mails_from_server,
        mail_handler::add_mail,
        mail_handler::get_all_mails,
        mail_handler::update_mail,
        mail_handler::delete_mail,
        bounce_logs_handler::handle_sns_notification,
        bounce_logs_handler::get_all_bounces,
        bounce_logs_handler::get_bounces_by_contact_id,
        bounce_logs_handler::delete_bounce,
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
        .nest("/mails", mail_routes::mail_routes())
        .nest("/bounce-logs", bounce_logs_route::bounce_logs_routes())
        .nest("/campaigns", campaign_routes::campaign_routes())
        .nest("/campaign-senders", campaign_senders_routes::campaign_sender_routes())
        .nest("/servers", servers_routes())
        .nest("/health", healthcheck_routes());

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/mail-service", ApiDoc::openapi())
        )
}
