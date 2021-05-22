use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    display_name: String,
    school: String,
}
