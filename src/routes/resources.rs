use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::postgres::PgPool;

use crate::data::resources::{add_resource, get_resource_by_id};
use crate::models::resources::NewResource;

#[get("/{chapter_id}/resources")]
async fn get_resources_of_subject(
    web::Path(chapter_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/resources/{resource_id}")]
async fn get_resource(
    web::Path(resource_id): web::Path<String>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let resource = get_resource_by_id(db.get_ref(), resource_id).await.unwrap();
    web::Json(resource)
}

#[post("/{chapter_id}/resources")]
async fn create_resource(
    web::Path(chapter_id): web::Path<String>,
    new_resource: web::Json<NewResource>,
    db: web::Data<PgPool>,
) -> impl Responder {
    let id = add_resource(db.get_ref(), new_resource.into_inner())
        .await
        .unwrap();
    HttpResponse::Ok()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_resource);
}
