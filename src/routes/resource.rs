use actix_web::{get, web, Responder};
use sqlx::postgres::PgPool;

use crate::data::resource::get_resource_by_id;

#[get("/{resource_id}")]
async fn get_resource(
    web::Path(resource_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let resource = get_resource_by_id(db.get_ref(), resource_id).await.unwrap();
    web::Json(resource)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_resource);
}
