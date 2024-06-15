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

pub mod table;
