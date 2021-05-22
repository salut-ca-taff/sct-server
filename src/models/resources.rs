use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Resource {
    pub id: i32,
    pub chapter: i32,
    pub author: i32,
    pub title: String,
    pub content: String,
    pub attachments: String,
    pub view_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewResource {
    pub author: i64,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ResourceId(pub i64);
