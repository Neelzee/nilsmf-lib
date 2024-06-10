use crate::models::{img::Image, tag::Tag};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub created: String,
    pub article_id: usize,
    pub images: Vec<Image>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MetaArticle {
    pub article_id: usize,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleTable {
    pub created: String,
    pub article_id: usize,
    pub images_id: usize,
    pub edited_date: Option<String>,
}
