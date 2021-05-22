use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Resource {
    pub id: i64,
    pub author: i64,
    pub description: String,
    // pub attachements: Option<Vec<String>>,
    // pub comments: Vec<i32>,
    //pub stars: (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>),
    pub views: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewResource {
    pub author: i64,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ResourceId(pub i64);
