use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: i32,
    pub subject: i32,
    pub title: String,
    pub description: String,
}
