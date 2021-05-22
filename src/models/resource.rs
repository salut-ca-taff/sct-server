use chrono::prelude::*;
use serde::Serialize;

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

#[derive(Debug)]
pub struct NewResource {
    pub author: i64,
    pub description: String,
    pub views: i64,
}
