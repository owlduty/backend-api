use dotenv::dotenv;
use grillon::Grillon;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub static GRILLON: Lazy<Grillon> = Lazy::new(|| {
    dotenv().ok();
    let api_base_url = env::var("API_BASE_URL").expect("`API_BASE_URL` is missing from .env file");

    Grillon::new(&api_base_url).expect("Failed to initialize Grillon instance to run tests")
});

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    env::var("JWT_SECRET").expect("`JWT_SECRET` is missing from .env file")
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

mod ctx;
mod jwt;
mod signin;
mod signup;
// mod synthetics;
