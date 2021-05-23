use sqlx::postgres::PgPool;

use crate::models::chapters::{Chapter, ChapterId, NewChapter};

// CHECK ERRORS

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

pub async fn get_chapter_from_id(db: &PgPool, chapter_id: String) {
    let chapter = sqlx::query(
        r#"
        SELECT chapter
        FROM chapters
        WHERE chapters.id = $1
        "#,
    )
    .bind(chapter_id)
    .fetch_one(db)
    .await?;

    Ok(chapter)
}

pub async fn add_chapter_to_subject(
    db: &PgPool,
    new_chapter: NewChapter,
    subject: i32,
) -> anyhow::Result<i32> {
    let ChapterId(id) = sqlx::query_as::<_, ChapterId>(
        r#"
        INSERT INTO subjects ( subject, title, description )
        VALUES ( $1, $2, $3 )
        RETURNING id;
        "#,
    )
    .bind(new_chapter.subject)
    .bind(new_chapter.title)
    .bind(new_chapter.description)
    .fetch_one(db)
    .await?;

    Ok(id)
}
