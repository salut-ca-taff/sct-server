use actix_web::{get, web, HttpResponse, Responder};

pub mod auth;
pub mod resources;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Salut ça taff API Index")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root)
        .service(web::scope("/auth").configure(auth::init))
        .service(web::scope("/").configure(resources::init));
}
