use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::postgres::PgPool;

use super::auth::AuthState;
use crate::models::course::{Course, NewCourse};

#[get("/{course_id}")]
async fn get_course(web::Path(course_id): web::Path<i32>, db: web::Data<PgPool>) -> impl Responder {
    let course = Course::find_by_id(db.get_ref(), course_id).await.unwrap();

    HttpResponse::Ok().json(course)
}

#[get("/{chapter_id}/courses")]
async fn get_courses(
    web::Path(chapter_id): web::Path<i32>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let courses = Course::find_all_from_chapter(db.get_ref(), chapter_id)
        .await
        .unwrap();

    HttpResponse::Ok().json(courses)
}

#[post("/{chapter_id}/courses")]
async fn create_course(
    web::Path(chapter_id): web::Path<i32>,
    new_course: web::Json<NewCourse>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let course = Course::add_to_chapter(db.get_ref(), chapter_id, new_course.into_inner())
        .await
        .unwrap();

    HttpResponse::Ok().json(course)
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.app_data(auth_state.clone()).service(
        web::scope("/")
            .wrap(HttpAuthentication::bearer(super::auth::auth_middleware))
            .configure(|cfg| {
                cfg.service(get_course)
                    .service(get_courses)
                    .service(create_course);
            }),
    );
}
