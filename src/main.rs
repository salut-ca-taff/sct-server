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
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    school VARCHAR(100) NOT NULL,
    avatar VARCHAR(100) NOT NULL
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    author INTEGER REFERENCES users(id) NOT NULL,
    content VARCHAR(512) NOT NULL,
    attachments VARCHAR(100) ARRAY[10],
    stars INTEGER[][5],
    created_at TIMESTAMP NOT NULL
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS resources (
    id SERIAL PRIMARY KEY,
    author INTEGER REFERENCES users(id) NOT NULL,
    title VARCHAR(100) NOT NULL,
    description VARCHAR(512),
    attachments VARCHAR(100) ARRAY[10],
    comments INTEGER[],
    stars INTEGER[][5],
    views INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS chapters (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    resources INTEGER[],
    prerequesites INTEGER[]
);
    "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS subjects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    chapters INTEGER[] NOT NULL
);
    "#,
    )
    .execute(db)
    .await?;

    Ok(())
}
