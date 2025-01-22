pub mod handlers { pub mod template; }

pub mod schema;
pub mod route;
pub mod model;
pub mod appState;

#[allow(warnings, unused)]
mod prisma;

use appState::AppState;

use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};

use route::create_router;
use tower_http::cors::CorsLayer;
use std::{net::SocketAddr, sync::Arc};

use dotenvy::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_url = if cfg!(test) {
        env::var("DATABASE_URL_TEST").unwrap_or_else(|_| {
            panic!("DATABASE_URL_TEST must be set when running tests")
        })
    } else {
        env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!("DATABASE_URL must be set")
        })
    };

    let pool = get_connection_pool(&database_url);

    let app_state = Arc::new(AppState::new(pool));

    // cors config...
    let cors = CorsLayer::new()
        .allow_origin(
            ["http://localhost:3000"]
            .iter()
            .map(|s| s.parse::<HeaderValue>().unwrap())
            .collect::<Vec<_>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(app_state).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // println!("Connected to the database: {:?}", client);
    println!("Server running at port: {}", &addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();

}


pub fn get_connection_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    println!("{}", &database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use when building a connection pool
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    pool
}