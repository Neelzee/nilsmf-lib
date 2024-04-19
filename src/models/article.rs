use crate::models::{img::Image, Tag};
use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub created: Date,
    pub article_id: usize,
    pub images: Vec<Image>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleTable {
    pub created: Date,
    pub article_id: usize,
    pub images_id: usize,
    pub edited_date: Option<Date>,
}
