use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root).service(login);
}
