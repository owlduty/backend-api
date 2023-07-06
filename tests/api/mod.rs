use async_trait::async_trait;
use dotenv::dotenv;
use grillon::Grillon;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::panic::{catch_unwind, AssertUnwindSafe};

pub static GRILLON: Lazy<Grillon> = Lazy::new(|| {
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

pub static PG_POOL: Lazy<PgPool> = Lazy::new(|| {
    dotenv().ok();
    let db_uri = env::var("DATABASE_URL").expect("`DATABASE_URL` is missing from .env file");

    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&db_uri)
        .expect("Failed to initialize PostgreSQL connection pool")
});

#[async_trait]
pub(crate) trait TestContext {
    async fn setup(&self);
    async fn teardown(&self);
    fn run_test<F>(&self, test: F) -> ()
    where
        F: std::future::Future,
    {
        TOKIO_RUNTIME.block_on(async { self.setup().await });

        let assert = AssertUnwindSafe(async { test.await });
        let result = catch_unwind(|| {
            TOKIO_RUNTIME.block_on(assert);
        });

        TOKIO_RUNTIME.block_on(async { self.teardown().await });

        assert!(result.is_ok());
    }
}

mod signup;
mod synthetics;
