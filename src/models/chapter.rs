use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: i32,
    pub subject: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct NewChapter {
    pub subject: i64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ChapterId(pub i64);
