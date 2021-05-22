use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User {
    id: Uuid,
    name: String,
    school: String,
    avatar: String,
}

impl User {}
