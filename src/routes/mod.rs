use actix_web::{get, web, HttpResponse, Responder};

use self::auth::AuthState;

pub mod auth;
pub mod chapters;
pub mod courses;
pub mod subjects;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Salut ça taff API Index")
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.service(root)
        .service(web::scope("/auth").configure(|cfg| auth::init(cfg, auth_state.clone())))
        .service(web::scope("/subjects").configure(|cfg| subjects::init(cfg, auth_state.clone())))
        .service(web::scope("/chapters").configure(|cfg| chapters::init(cfg, auth_state.clone())))
        .service(web::scope("/courses").configure(|cfg| courses::init(cfg, auth_state.clone())));
}
