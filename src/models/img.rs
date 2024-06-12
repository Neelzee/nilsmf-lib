use getset::Getters;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Getters)]
pub struct Image {
    title: String,
    tags: Vec<String>,
    img: Vec<u32>,
    alt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageTable {
    pub iid: usize,
    pub title: String,
    pub img: Vec<u32>,
}
