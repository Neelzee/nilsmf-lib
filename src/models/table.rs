use diesel::prelude::*;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters, Queryable, Selectable)]
#[diesel(table_name = article)]
pub struct ArticleTable {
    iid: u64,
    content: String,
    description: String,
}

diesel::table! {
    article (iid) {
        iid -> Int4,
        content -> Text,
        description -> Text
    }

    meta_article (iid) {
        iid -> Int4,
        title -> Text,
        created_date -> Varchar,
        created_time -> Varchar,
        last_edit_date -> Varchar,
        last_edit_time -> Varchar,
        tags -> Varchar[],
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, Queryable, Selectable)]
#[diesel(table_name = meta_article)]
pub struct MetaArticleTable {
    iid: u64,
    title: String,
    created_date: String,
    created_time: String,
    last_edit_date: Option<String>,
    last_edit_time: Option<String>,
    tags: Vec<String>,
    author_id: u64,        // <- AuthorTable
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
    add_tags: Vec<String>,
    rm_tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ImageTable {
    iid: u64,
    title: String,
    alt: String,
    src: Vec<u8>,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct ImageArticleTable {
    iid: u64,
    image_id: u64,
    article_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct AuthorTable {
    iid: u64,
    uid: u64,
    name: String,
    email: String,
}
