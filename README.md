# backend-api

The backend API served with PostgREST. This service is in charge of exposing the main API with
CRUD operations. It uses PostgreSQL to put as many restrictions as possible upstream. Here directly
on the database model.

## Prerequisites

- Rust
- Docker with docker-compose
- [Diesel CLI](https://crates.io/crates/diesel_cli)
  - `cargo install diesel_cli --no-default-features --features postgres`

## Getting Started

- `cp .env.example .env`
- `docker-compose up -d`
- `docker exec -it backend-api_postgres_1 psql -U dev -d backend_main` (to verify you can connect)
- `diesel setup`
