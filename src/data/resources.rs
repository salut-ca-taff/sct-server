use sqlx::postgres::PgPool;

use crate::models::resources::{NewResource, Resource, ResourceId};

pub async fn get_resource_by_id(db: &PgPool, id: String) -> anyhow::Result<Resource> {
    let resource = sqlx::query_as::<_, Resource>(
        r#"
SELECT resource as "resouce: Resource"
FROM resources
WHERE resource.id = $1;
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(resource)
}

pub async fn add_resource_to_chapter(
    db: &PgPool,
    new_resource: NewResource,
    chapter: i32,
) -> anyhow::Result<i32> {
    let ResourceId(id) = sqlx::query_as::<_, ResourceId>(
        r#"
INSERT INTO resources ( chapter, author, description )
VALUES ( $1, $2, $3 )
RETURNING id;
        "#,
    )
    .bind(chapter)
    .bind(new_resource.author)
    .bind(new_resource.description)
    .fetch_one(db)
    .await?;

    Ok(id)
}
