use std::env;
use std::panic::{catch_unwind, AssertUnwindSafe};

use dotenv::dotenv;
use grillon::Grillon;
use once_cell::sync::Lazy;

pub static GRILLON: Lazy<Grillon> = Lazy::new(|| {
    // load .env variables
    dotenv().ok();
    let api_base_url = env::var("API_BASE_URL").expect("`API_BASE_URL` is missing from .env file");

    Grillon::new(&api_base_url).expect("Failed to initialize Grillon instance to run tests")
});

pub static TOKIO_RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize tokio runtime")
});

pub(crate) trait TestContext {
    fn setup();
    fn teardown();
    fn run_test<F>(test: F) -> ()
    where
        F: std::future::Future,
    {
        Self::setup();

        let assert = AssertUnwindSafe(async { test.await });
        let result = catch_unwind(|| {
            TOKIO_RUNTIME.block_on(assert);
        });

        Self::teardown();

        assert!(result.is_ok());
    }
}

mod auth;
mod synthetics;
