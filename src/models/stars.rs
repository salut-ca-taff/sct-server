use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Stars {
    pub id: i32,
    pub value: i32,
    pub user: i32,
    pub resource: i32,
}
