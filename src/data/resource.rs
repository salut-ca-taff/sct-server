use sqlx::postgres::PgPool;

use crate::models::resource::Resource;

pub async fn get_resource_by_id(db: &PgPool, id: String) -> anyhow::Result<Resource> {
    let resource = sqlx::query_as::<_, Resource>(
        r#"
SELECT resource as "resouce: Resource"
FROM resources
WHERE resource.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(resource)
}
