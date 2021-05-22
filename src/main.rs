#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use std::env;

use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use dotenv::dotenv;
use futures::TryStreamExt;
use log::info;
use reql::{cmd::connect::Options, Session};

#[get("/")]
async fn root(db: Data<Session>) -> impl Responder {
    let infos: Option<i32> = reql::r
        .now()
        .day_of_week()
        .run(db.as_ref())
        .try_next()
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("Salut ca taff ? {:?}", infos))
}

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").context("HOST environment variable is not set")?;
    let port = env::var("PORT").context("PORT environment variable is not set")?;

    let mut database_options = Options::default();
    if let Ok(database_host) = env::var("DATABASE_HOST") {
        database_options = database_options.host(database_host);
    }
    if let Ok(database_port) = env::var("DATABASE_PORT") {
        database_options = database_options.host(database_port);
    }
    let conn = reql::r
        .connect(database_options)
        .await
        .context("Couldn't connect to RethinkDB database")?;

    let server = HttpServer::new(move || {
        App::new()
            .data(conn.clone())
            .wrap(Logger::default())
            .service(root)
    });

    info!("Starting server...");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
