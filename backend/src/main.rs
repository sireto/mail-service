use backend::route::create_router;
use diesel::PgConnection;
use diesel::Connection; // Import the Connection trait
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;
use std::{env, net::SocketAddr};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Set up database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let mut connection = establish_connection(&database_url);

    // Run migrations
    run_migrations(&mut connection);

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(
            ["http://localhost:3000", "http://localhost:8000"]
                .iter()
                .map(|s| s.parse::<HeaderValue>().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Axum app
    let app = create_router().layer(cors);

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
    // connection.run_migration(MIGRATIONS).expect("Failed to run database migrations");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending database migrations");

    println!("{:?}",connection.applied_migrations());

}
