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

#[derive(Debug, Deserialize)]
pub struct NewCourse {
    author: String,
    title: String,
    small: Option<bool>,
    level: String,
    content: Option<String>,
}

impl Course {
    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Self> {
        query_as("SELECT * FROM courses WHERE id = $1")
            .bind(id)
            .fetch_one(db)
            .await
            .context("Couldn't fetch course by id")
    }

    pub async fn find_all_from_chapter(db: &PgPool, chapter_id: i32) -> Result<Vec<Self>> {
        query_as("SELECT * FROM courses where chapter = $1")
            .bind(chapter_id)
            .fetch_all(db)
            .await
            .context("Couldn't fetch courses from chapter")
    }

    pub async fn add_to_chapter(db: &PgPool, chapter: i32, new_course: NewCourse) -> Result<Self> {
        let mut additions = (String::default(), String::default());

        if new_course.small.is_some() {
            additions.0 += ", small";
            additions.1 += ", $4";
            if new_course.content.is_some() {
                additions.0 += ", content";
                additions.1 += ", $5";
            }
        } else {
            if new_course.content.is_some() {
                additions.0 += ", content";
                additions.1 += ", $4";
            }
        }

        let query = format!("INSERT INTO courses (chapter, author, title, level{}) VALUES ($1, $2, $3, $4{}) RETURNING *", additions.0, additions.1);
        let mut query = query_as(&query)
            .bind(chapter)
            .bind(new_course.author)
            .bind(new_course.title)
            .bind(new_course.level);

        if let Some(small) = new_course.small {
            query = query.bind(small);
        }
        if let Some(content) = new_course.content {
            query = query.bind(content);
        }

        query
            .fetch_one(db)
            .await
            .context("Couldn't insert a new course to chapter")
    }
}
