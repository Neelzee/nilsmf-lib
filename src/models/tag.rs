use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagTable {
    pub iid: usize,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsTable {
    pub iid: usize,
    pub tag_id: usize,
    pub content_id: usize,
}
