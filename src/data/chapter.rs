use sqlx::postgres::PgPool;

use crate::models::chapters::Chapter;

pub async fn get_chapters_from_subject(db: &PgPool, subjectId: String) -> anyhow::Result<Resource> {
    let chapters: Vec<chapter> = sqlx::query(
        r#"
        SELECT * IN chapters
        WHERE subject = $1
        "#,
    )
    .bind(subjectId)
    .fetch(db)
    .await?;

    Ok(chapters)
}
