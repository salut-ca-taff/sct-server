use chrono::prelude::*

struct Comment {
    id: Uuid,
    author: String,
    content: String,
    attachements: Option<Vec<String>>,
    stars: (Vec<User>, Vec<User>, Vec<User>, Vec<User>),
    date: DateTime<Utc>,
}

impl Comment {
}


