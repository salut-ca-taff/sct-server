use super::auth::AuthState;
use crate::models::chapter::Chapter;
use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;

#[get("/{chapter}")]
async fn get_chapter(web::Path(id): web::Path<i32>, db: web::Data<PgPool>) -> impl Responder {
    let chapter = Chapter::find_by_id(db.as_ref(), id).await.unwrap();
    HttpResponse::Ok().json(chapter)
}

#[get("")]
async fn get_chapters(db: web::Data<PgPool>) -> impl Responder {
    let chapters = Chapter::find_all(db.as_ref()).await.unwrap();
    HttpResponse::Ok().json(chapters)
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.app_data(auth_state.clone()).service(
        web::scope("")
            .wrap(HttpAuthentication::bearer(super::auth::auth_middleware))
            .configure(|cfg| {
                cfg.service(get_chapters).service(get_chapter);
            }),
    );
}
