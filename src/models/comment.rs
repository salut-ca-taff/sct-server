use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Comment {
    pub id: i32,
    pub author: String,
    pub course: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

impl Comment {}
