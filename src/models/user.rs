use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: u64,
    name: String,
    school: String,
    avatar: String,
}
