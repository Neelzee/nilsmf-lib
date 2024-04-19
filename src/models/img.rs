use crate::models::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub title: String,
    pub tags: Vec<Tag>,
    pub img: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageTable {
    pub iid: usize,
    pub title: String,
    pub img: Vec<u32>,
}
