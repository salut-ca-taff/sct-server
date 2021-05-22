use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;

#[derive(Deserialize, Serialize)]
pub struct Comment {
    id: Uuid,
    author: String,
    content: String,
    attachements: Option<Vec<String>>,
    stars: (Vec<User>, Vec<User>, Vec<User>, Vec<User>),
    date: DateTime<Utc>,
}

impl Comment {}
