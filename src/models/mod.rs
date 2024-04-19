use serde::{Deserialize, Serialize};

/// Contains Article and ArticleTable
pub mod article;

/// Contains Image and ImageTable
pub mod img;

/// Contains Tag, TagTable and TagsTable
pub mod tag;

/// Contains User and Email
pub mod user;

#[derive(Serialize, Deserialize)]
/// Wrapper for data being sent
pub struct Wrapper<T> {
    iid: String,
    content: T,
}
