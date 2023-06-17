use std::env;

use dotenv::dotenv;
use grillon::Grillon;
use once_cell::sync::Lazy;

pub static GRILLON: Lazy<Grillon> = Lazy::new(|| {
    // load .env variables
    dotenv().ok();
    let api_base_url = env::var("API_BASE_URL").expect("`API_BASE_URL` is missing from .env file");

    Grillon::new(&api_base_url).expect("Failed to initialize Grillon instance to run tests")
});

mod synthetics;
mod users;
