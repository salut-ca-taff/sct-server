use sqlx::postgres::PgPool;

use crate::models::chapters::Chapter;

pub async fn get_chapters_from_subject(db: &PgPool, subjectId: String) -> anyhow::Result<Resource> {
    let result: Vec<Chapter> = sqlx::query(
        r#"
        SELECT chapter FROM subjects
        "#,
    )
    .fetch()
    .await?;
    result
}
