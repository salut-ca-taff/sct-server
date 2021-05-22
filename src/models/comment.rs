use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Comment {
    id: i32,
    author: i32,
    content: String,
    attachements: String,
    created_at: DateTime<Utc>,
}

impl Comment {}
