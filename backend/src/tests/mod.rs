use dotenv::dotenv;
use std::sync::{Arc, Once};

static INIT: Once = Once::new();

pub fn setup_test_environment() {
    INIT.call_once(|| {
        dotenv().ok(); // Load environment variables from .env.test
        std::env::set_var("RUST_BACKTRACE", "1");
    });
}
