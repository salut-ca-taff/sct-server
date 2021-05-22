#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use futures::TryStreamExt;
use reql::{cmd::connect::Options, r, types::ServerStatus, Session};
use std::error;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Salut ca taff ?")
}

const RETHINKDB_PORT: u16 = 28015;
const SERVER_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let session = r.connect(Options::new().port(RETHINKDB_PORT)).await?;

    if let Err(e) = r
        .table_create("taff")
        .run::<&Session, ServerStatus>(&session)
        .try_next()
        .await
    {
        match e {
            reql::Error::Runtime(reql::Runtime::Availability(_)) => (),
            e => Err(e)?,
        }
    }

    HttpServer::new(move || App::new().data(session.clone()).service(root))
        .bind(format!("127.0.0.1:{}", SERVER_PORT))?
        .run()
        .await?;

    Ok(())
}
