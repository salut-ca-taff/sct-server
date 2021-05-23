use actix_web::{get, web, Responder};
use sqlx::postgres::PgPool;

use crate::data::chapters::get_chapters_from_subject;
// get_chapters_by_subject_id
// get chapter id by subject id
// create (post) chapter from subject id

#[get("/{subject_id}")]
async fn get_chapters(
    web::Path(subject_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let chapters = get_chapters_from_subject(db.get_ref(), subject_id);
    web::Json(chapters)
}

#[get("/chapters/{chapter_id}")]
async fn get_chapter(
    web::Path(chapter_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let chapter = get_chapter_from_id(db.get_ref(), subject_id).await.unwrap();
    web::Json(chapter)
}

#[post("/{subject_id}/chapters")]
async fn create_chapter(
    web::Path(subject_id): web::Path<String>,
    new_chapter: web::Json<NewChapter>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let id = add_chapter_to_subject(db.get_ref(), new_chapter.into_inner(), subject_id)
        .await
        .unwrap();
    web::Json(ChapterId(id))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chapters);
}
