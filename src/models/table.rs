use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ArticleTable {
    iid: u64,
    content: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct MetaArticleTable {
    iid: u64,
    title: String,
    created_date: String,
    created_time: String,
    last_edit_date: Option<String>,
    last_edit_time: Option<String>,
    author_id: u64,
    content_table_id: u64, // <- ArticleTable
}

/// Table containing edit information of an article
#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct MetaArticleEditTable {
    iid: u64,
    edit_date: String,
    edit_time: String,
    article_id: u64, // <- ArticleTable
    changes_id: u64, // <- ArticleEditTable
}

/// Table containing edit changes of an article
#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ArticleEditTable {
    iid: u64,
    new_content: String,
    add_images: u64,
    rm_images: u64,
    add_tags: u64,
    rm_tags: u64,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ImageTable {
    iid: u64,
    title: String,
    alt: String,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ImageArticleTable {
    iid: u64,
    image_id: u64,
    article_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct TagTable {
    iid: u64,
    tag: String,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ImageTagTable {
    iid: u64,
    image_id: u64,
    tag_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ArticleTagTable {
    iid: u64,
    article_id: u64,
    tag_id: u64,
}
