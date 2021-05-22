use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::comment::Comment;
use crate::models::user::User;

#[derive(Deserialize, Serialize)]
pub struct Resource {
    id: Uuid,
    author: String,
    description: String,
    attachements: Option<Vec<String>>,
    comments: Vec<Comment>,
    stars: (Vec<User>, Vec<User>, Vec<User>, Vec<User>, Vec<User>),
    views: usize,
    data: DateTime<Utc>,
}
