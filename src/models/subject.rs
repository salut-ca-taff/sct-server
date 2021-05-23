use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subject {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub color: String,
}
