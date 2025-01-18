pub mod handlers { pub mod template; }

mod schema;
mod route;
mod model;
mod appState;

#[allow(warnings, unused)]
mod prisma;

use appState::AppState;
use prisma::PrismaClient;
use prisma_client_rust::NewClientError;


use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};

use route::create_router;
use tower_http::cors::CorsLayer;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let client = PrismaClient::_builder().build().await.unwrap();

    let app_state = Arc::new(AppState { db: client });

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
    axum::serve(listener, app.into_make_service()).await.unwrap();

}
