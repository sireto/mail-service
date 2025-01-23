use std::env;
use dotenv::dotenv;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup_test_environment() {
    INIT.call_once(|| {
        // Load environment variables from .env from the root...
        dotenv().ok(); 
        
        env::set_var("RUST_BACKTRACE", "1");

        // set the environment variable DATABASE_URL to DATABASE_URL_TEST for testing purposes...
        env::set_var("DATABASE_URL", env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set"));    
    });
}