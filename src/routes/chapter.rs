use actix_web::{get, web, Responder};
use sqlx::postgres::PgPool;

use crate::data::chapters::get_chapters_from_subject;
// get_chapters_by_subject_id
// get chapter id by subject id
// create (post) chapter from subject id

#[get("/{subject_id}")]
async fn get_chapters(
    web::Path(chapter_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let chapters = get_chapters_from_subject(db.get_ref(), subject_id);
    web::Json(chapters)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chapters);
}
