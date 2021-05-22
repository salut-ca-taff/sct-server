#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use reql::{cmd::connect::Options, r};
use std::error::Error;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Salut ca taff ?")
}

const RETHINKDB_PORT: u16 = 28015;
const SERVER_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let session = r.connect(Options::new().port(RETHINKDB_PORT)).await?;

    HttpServer::new(move || App::new().data(session.clone()).service(root))
        .bind(format!("127.0.0.1:{}", SERVER_PORT))?
        .run()
        .await?;

    Ok(())
}
