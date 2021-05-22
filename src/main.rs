#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

mod routes;

#[get("/")]
async fn root(_db: Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().body(format!("Salut ca taff ?"))
}

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
