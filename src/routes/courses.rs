use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use super::auth::AuthState;

/*#[get("/{chapter_id}/courses")]
async fn get_courses_of_subject(
    web::Path(chapter_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/courses/{course_id}")]
async fn get_course(
    web::Path(course_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let course = get_course_by_id(db.get_ref(), course_id).await.unwrap();
    web::Json(course)
}

#[post("/{chapter_id}/courses")]
async fn create_course(
    web::Path(chapter_id): web::Path<i32>,
    new_course: web::Json<NewCourse>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let id = add_course_to_chapter(db.get_ref(), new_course.into_inner(), chapter_id)
        .await
        .unwrap();
    web::Json(CourseId(id))
}*/

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.app_data(auth_state.clone()).service(
        web::scope("/").wrap(HttpAuthentication::bearer(super::auth::auth_middleware)), /*.configure(|cfg| {
                                                                                            cfg.service(get_course);
                                                                                        })*/
    );
}
