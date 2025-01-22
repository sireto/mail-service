use std::{net::SocketAddr, sync::Arc};
use once_cell::sync::Lazy;

use backend::tests::setup_test_environment;
use backend::db::{establish_connection, DbPool};
use backend::route::create_router;
use backend::appState::AppState;

pub static TEST_ENV: Lazy<TestEnv> = Lazy::new(|| {
    setup_test_environment();
    TestEnv::new()
});

pub struct TestEnv {
    pub pool: DbPool,
    pub addr: SocketAddr,
}

impl TestEnv {
    pub fn new() -> Self {
        let pool = establish_connection();
        let app_state = Arc::new(AppState::new(pool.clone()));
        let app = create_router(app_state.clone());
        let addr = SocketAddr::from(([127, 0, 0, 1], 9000)); // Tests run at port 9000

        // Start the server in the background
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app.into_make_service()).await.unwrap();
        });

        Self { pool, addr }
    }
}