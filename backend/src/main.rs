use axum::middleware;
use backend::error::AppError;
use backend::repositories::mail_repository;
use backend::route::create_router;
use backend::services::mail_service::{self, MailServiceTrait};
use backend::servers::{ servers_repo, servers_services };
use diesel::PgConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method
};
use tower_http::cors::CorsLayer;
use std::{env, net::SocketAddr, sync::Arc};
use backend::middleware::error_handling_middleware;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    // Set up database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let mut connection = establish_connection(&database_url);

    // Run migrations
    run_migrations(&mut connection);

    let binding = env::var("ORIGINS").expect("ORIGINS must be set in .env file in the comma separated strings format");
    let origins = binding.split(',').collect::<Vec<&str>>();

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(
            origins
                .iter()
                .map(|s| s.parse::<HeaderValue>().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Axum app
    let app = create_router()
        .layer(cors)
        .layer(middleware::from_fn(error_handling_middleware));

    // Instantiate the server service and repository one time, and inject it to the process_mails background process...
    let server_repo = Arc::new(servers_repo::ServerRepoImpl);
    let server_service = Arc::new(servers_services::ServerService::new(server_repo));

    let mail_repo = Arc::new(mail_repository::MailRepositoryImpl);
    let mail_service = Arc::new(mail_service::MailService::new(mail_repo));

    // Worker for processing mails...
    {
        let mail_service = Arc::clone(&mail_service);
        let server_service = Arc::clone(&server_service);
        tokio::spawn(async move {
            if let Err(err) = mail_service.process_mails(server_service.into()).await.map_err(|err| AppError::InternalServerError(Some(format!("Mail worker error: {:?}", err.to_string())))) {
                eprintln!("Error occurred in mail worker: {:?}", err);
            }
        });
    }

    // Address configuration
    let addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid server address");
    println!("Starting server at {addr}");

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

/// Establishes a database connection to PostgreSQL
fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to PostgreSQL database at {}", database_url))
}

/// Runs the migrations on PostgreSQL
fn run_migrations(connection: &mut impl MigrationHarness<diesel::pg::Pg>) {
    connection
    .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending migrations");
    println!("{:?}",connection.applied_migrations());
}
