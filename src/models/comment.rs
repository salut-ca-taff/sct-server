use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Comment {
    id: u64,
    author: String,
    content: String,
    attachements: Option<Vec<String>>,
    stars: (Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>),
    date: DateTime<Utc>,
}

impl Comment {}
