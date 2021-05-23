use super::auth::AuthState;
use crate::models::{chapter::Chapter, subject::Subject};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::{query_as, PgPool};

#[get("")]
async fn get_subjects(db: Data<PgPool>) -> impl Responder {
    let subjects: Vec<Subject> = query_as("SELECT * FROM subjects")
        .fetch_all(db.as_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(subjects)
}

#[get("/{subject}")]
async fn get_subject(web::Path(subject): web::Path<i32>, db: Data<PgPool>) -> impl Responder {
    let subject: Subject = query_as("SELECT * FROM subjects WHERE id = $1")
        .bind(subject)
        .fetch_one(db.as_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(subject)
}

#[get("/{subject}/chapters")]
async fn get_chapters_by_subject(
    web::Path(subject): web::Path<i32>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let chapters = Chapter::find_all_from_subject(db.as_ref(), subject)
        .await
        .unwrap();
    HttpResponse::Ok().json(chapters)
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.app_data(auth_state.clone()).service(
        web::scope("")
            .wrap(HttpAuthentication::bearer(super::auth::auth_middleware))
            .configure(|cfg| {
                cfg.service(get_subjects)
                    .service(get_subject)
                    .service(get_chapters_by_subject);
            }),
    );
}
