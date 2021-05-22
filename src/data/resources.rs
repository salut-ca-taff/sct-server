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

pub async fn add_resource(db: &PgPool, new_resource: NewResource) -> anyhow::Result<i64> {
    let ResourceId(id) = sqlx::query_as::<_, ResourceId>(
        r#"
INSERT INTO resources ( author, description )
VALUES ( $1, $2 )
RETURNING id;
        "#,
    )
    .bind(new_resource.author)
    .bind(new_resource.description)
    .fetch_one(db)
    .await?;

    Ok(id)
}
