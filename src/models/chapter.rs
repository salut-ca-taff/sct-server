#[derive(Debug)]
pub struct Chapter {
    pub id: i64,
    pub resources: Vec<resource>,
    pub prerequisite: Vec<Chapter>,
}
