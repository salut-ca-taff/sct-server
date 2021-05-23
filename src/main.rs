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
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::routes::auth::AuthState;

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

    let auth_state = actix_web::web::Data::new(AuthState::default());

    let server = HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(Logger::default())
            .configure(|cfg| routes::init(cfg, auth_state.clone()))
    });

    info!("Starting server...");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
