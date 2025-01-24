use backend::route::create_router;

use dotenv::dotenv;

use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};

use tower_http::cors::CorsLayer;
use std::{net::SocketAddr, env};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // cors config...
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

    let app = create_router().layer(cors);

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    let addr: SocketAddr = addr.parse().expect("Invalid server address");

    println!("THE Actual socket addr ===> {addr:?}");


    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();

}