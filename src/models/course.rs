use anyhow::{Context, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub chapter: i32,
    pub author: String,
    pub title: String,
    pub small: bool,
    pub level: String,
    pub content: String,
    pub view_count: i32,
    pub created_at: DateTime<Utc>,
}

impl Course {
    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Self> {
        let course = query_as("SELECT * FROM courses WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await?;

        Ok(course)
    }

    pub async fn add_to_chapter(
        db: &PgPool,
        chapter: i32,
        author: String,
        title: String,
        small: Option<bool>,
        level: String,
        content: Option<String>,
    ) -> Result<Self> {
        let mut additions = (String::default(), String::default());

        if small.is_some() {
            additions.0 += ", small";
            additions.1 += ", $4";
            if content.is_some() {
                additions.0 += ", content";
                additions.1 += ", $5";
            }
        } else {
            if content.is_some() {
                additions.0 += ", content";
                additions.1 += ", $4";
            }
        }

        let query = format!("INSERT INTO courses (chapter, author, title, level{}) VALUES ($1, $2, $3, $4{}) RETURNING *", additions.0, additions.1);
        let mut query = query_as(&query)
            .bind(chapter)
            .bind(author)
            .bind(title)
            .bind(level);

        if let Some(small) = small {
            query = query.bind(small);
        }
        if let Some(content) = content {
            query = query.bind(content);
        }

        query
            .fetch_one(db)
            .await
            .context("Couldn't insert a new course to chapter")
    }
}
