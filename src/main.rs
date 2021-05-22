#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use actix_web::{middleware::Logger, App, HttpServer};
use anyhow::{Context, Result};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

mod data;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").context("HOST environment variable is not set")?;
    let port = env::var("PORT").context("PORT environment variable is not set")?;
    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL environment variable is not set")?;

    let db = PgPoolOptions::new().connect(&database_url).await?;
    create_tables(&db).await?;

    let server = HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(Logger::default())
            .configure(routes::init)
    });

    info!("Starting server...");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}

async fn create_tables(db: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL NOT NULL,
    username VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    school VARCHAR(100) NOT NULL,
    avatar VARCHAR(100) NOT NULL,
    PRIMARY KEY (id)
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS comments (
    id BIGSERIAL NOT NULL,
    author BIGINT REFERENCES users(id) NOT NULL,
    content VARCHAR(512) NOT NULL,
    attachments VARCHAR(100) ARRAY[10],
    -- stars BIGINT[][5],
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS resources (
    id BIGSERIAL NOT NULL,
    author BIGINT REFERENCES users(id) NOT NULL,
    title VARCHAR(100) NOT NULL,
    description VARCHAR(512),
    attachments VARCHAR(100) ARRAY[10],
    -- comments BIGINT[],
    -- stars BIGINT[][5],
    views BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS chapters (
    id BIGSERIAL NOT NULL,
    name VARCHAR(100) NOT NULL,
    resources BIGINT[],
    prerequesites BIGINT[],
    PRIMARY KEY (id)
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS subjects (
    id VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    chapters BIGINT[] NOT NULL,
    PRIMARY KEY (id)
);
    "#,
    )
    .execute(db)
    .await?;

    Ok(())
}
