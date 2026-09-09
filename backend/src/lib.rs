use app_state::DbPooledConnection;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;

pub mod models {
    pub mod bounce_logs;
    pub mod campaign;
    pub mod campaign_lists;
    pub mod campaign_sender;
    pub mod contact;
    pub mod list;
    pub mod list_contacts;
    pub mod mail;
    pub mod pagination;
    pub mod template;
}
pub mod handlers {
    pub mod bounce_logs_handler;
    pub mod campaign;
    pub mod campaign_sender;
    pub mod contact;
    pub mod healthcheck_handler;
    pub mod list;
    pub mod mail_handler;
    pub mod template;
}
pub mod services {
    pub mod aws_service;
    pub mod bounce_logs_service;
    pub mod campaign_sender_service;
    pub mod campaign_service;
    pub mod contact_service;
    pub mod list_service;
    pub mod mail_service;
    pub mod template_service;
}
pub mod repositories {
    pub mod bounce_logs_repo;
    pub mod campaign;
    pub mod campaign_lists_repo;
    pub mod campaign_sender;
    pub mod contact;
    pub mod list_contact_repo;
    pub mod list_repo;
    pub mod mail_repository;
    pub mod template_repo;
}
pub mod routes {
    pub mod bounce_logs_route;
    pub mod campaign;
    pub mod campaign_senders;
    pub mod contact;
    pub mod healthcheck_route;
    pub mod list;
    pub mod mail;
    pub mod template;
}
pub mod servers {
    pub mod servers_handler;
    pub mod servers_model;
    pub mod servers_repo;
    pub mod servers_routes;
    pub mod servers_services;
}

pub mod app_state;
pub mod error;
pub mod route;
pub mod schema;
pub mod utils {
    pub mod bounce_logs;
    pub mod contact_lists_functions;
    pub mod crypto;
    pub mod email_utils;
    pub mod mjml_parser;
    pub mod server_utils;
    pub mod template_utils;
}
pub mod middleware;

use crate::app_state::AppState;
use once_cell::sync::Lazy;
use std::sync::Arc;

/**
 * Establish a connection to the database (being used for the testing purpose)...
 */
pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set when running tests"));

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub static GLOBAL_APP_STATE: Lazy<Arc<AppState>> = Lazy::new(|| {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set when running tests"));

    println!("THe database url is ====> {}", database_url.clone());

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Failed to create DB pool");

    Arc::new(AppState::new(db_pool))
});

pub async fn get_connection_pool() -> DbPooledConnection {
    GLOBAL_APP_STATE
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool")
}
