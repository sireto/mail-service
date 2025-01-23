use diesel::{prelude::*, r2d2::{
    ConnectionManager, 
    Pool 
}};
use dotenv::dotenv;
use std::env;

pub mod handlers { pub mod template; }
pub mod services { pub mod template_service; }
pub mod repositories { pub mod template_repo; }
pub mod tests;

pub mod schema;
pub mod route;
pub mod model;
pub mod appState;

use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::appState::AppState;

/**
 * Establish a connection to the database (being used for the testing purpose)...
 */
pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url =  env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL must be set when running tests")
    });

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    pool
}

pub static GLOBAL_APP_STATE: Lazy<Arc<AppState>> = Lazy::new(|| {
    dotenv().ok();

    let database_url =  env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL must be set when running tests")
    });

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Failed to create DB pool");
    
    Arc::new(AppState::new(db_pool))
});