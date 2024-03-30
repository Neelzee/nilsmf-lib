use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub created: Date,
    pub article_id: String,
    pub images: Vec<Image>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub title: String,
    pub tags: Vec<Tag>,
    pub img: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
}
