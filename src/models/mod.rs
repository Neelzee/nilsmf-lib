use rocket::response::Responder;
use serde::{Deserialize, Serialize};

/// Contains Article and ArticleTable
pub mod article;

/// Contains Image and ImageTable
pub mod img;

/// Contains User and Email
pub mod user;

/// Contains Version and Runtime
pub mod meta;

#[derive(Serialize, Deserialize)]
/// Wrapper for data being sent
pub struct Wrapper<T> {
    content: T,
    iid: String,
    content_type: String,
}
