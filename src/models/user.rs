use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub username: String,
    pub salt: String,
    pub password: String,
    pub school: String,
}
