use crate::models::img::Image;
use getset::Getters;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Getters)]
pub struct Article {
    content: String,
    #[schema(example = "11.06.24 [23:37]")]
    created: String,
    #[schema(example = "13.06.24 [00:06]")]
    last_edited: Vec<String>,
    tags: Vec<String>,
    author: String,
    images: Vec<Image>,
}
