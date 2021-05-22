use actix_web::{get, web, HttpResponse, Responder};

pub mod auth;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Salut ça taff API Index")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root);
    cfg.service(web::scope("/auth").configure(auth::init));
}
