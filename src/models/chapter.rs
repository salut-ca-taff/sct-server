use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: i32,
    pub subject: i32,
    pub title: String,
    pub description: String,
}

impl Chapter {
    pub async fn find_all(db: &PgPool) -> Result<Vec<Self>> {
        query_as("SELECT * IN chapters")
            .fetch_all(db)
            .await
            .context("Couldn't fetch all chapters")
    }

    pub async fn find_all_from_subject(db: &PgPool, subject: i32) -> Result<Vec<Self>> {
        query_as("SELECT * IN chapters WHERE subject = $1")
            .bind(subject)
            .fetch_all(db)
            .await
            .context("Couldn't fetch all chapters from subject")
    }

    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Self> {
        query_as("SELECT * FROM chapters WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await
            .context("Couldn't fetch chapter from id")
    }
}
