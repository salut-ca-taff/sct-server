use sqlx::postgres::PgPool;

use crate::models::chapters::Chapter;

pub async fn get_chapters_from_subject(db: &PgPool, subjectId: String) -> anyhow::Result<Resource> {
    // black Sql magic to JOIN subjects and chapters :wizard:
    // NOT IMPLEMENTED
    let chapters: Vec<chapter> = sqlx::query(
        r#"
        SELECT chapters
        FROM subjects
        WHERE subjects.id = $1
        "#,
    )
    .bind(subjectId)
    .fetch(db)
    .await?;

    Ok(chapters)
}
