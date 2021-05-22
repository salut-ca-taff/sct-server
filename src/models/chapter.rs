use serde::Serialize;
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: i64,
    pub resources: Vec<i64>,
    pub prerequisite: Vec<i64>,
}
