use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    id: u64,
    name: String,
    school: String,
    avatar: String,
}
