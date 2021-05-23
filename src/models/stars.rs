use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Stars {
    pub id: i32,
    pub value: i32,
    pub user: String,
    pub course: i32,
}
