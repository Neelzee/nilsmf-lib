use diesel::{dsl::Nullable, prelude::*, sql_types::Float};
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

diesel::table! {
    meta_article (iid) {
        iid -> Int4,
        title -> Text,
        created_date -> VarChar,
        created_time -> VarChar,
        last_edit_date -> VarChar,
        last_edit_time -> VarChar,
        tags -> Array<VarChar>,
        author_id -> Int4,
        content_table_id -> Int4,
    }

}

#[derive(Debug, Serialize, Deserialize, Getters, Queryable, Selectable)]
#[diesel(table_name = meta_article_edit)]
pub struct MetaArticleEditTable {
    iid: u64,
    edit_date: String,
    edit_time: String,
    article_id: u64, // <- ArticleTable
    changes_id: u64, // <- ArticleEditTable
}

diesel::table! {
    meta_article_edit (iid) {
        iid -> Int4,
        edit_date -> VarChar,
        edit_time -> VarChar,
        article_id -> Int4,
        changes_id -> Int4,
    }
}

/// Table containing edit changes of an article
#[derive(Debug, Serialize, Deserialize, Getters, Queryable, Selectable)]
#[diesel(table_name = article_edit)]
pub struct ArticleEditTable {
    iid: u64,
    new_content: String,
    add_images: Vec<u64>,
    rm_images: Vec<u64>,
    add_tags: Vec<String>,
    rm_tags: Vec<String>,
}

diesel::table! {
    article_edit (iid) {
        iid -> Int4,
        new_content -> VarChar,
        add_images -> Array<Int4>,
        rm_images -> Array<Int4>,
        add_tags -> Array<VarChar>,
        rm_tags -> Array<VarChar>,
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, Queryable, Selectable)]
#[diesel(table_name = image)]
pub struct ImageTable {
    iid: u64,
    title: String,
    alt: String,
    src: Vec<u8>,
    tags: Vec<String>,
}

diesel::table! {
    image (iid) {
        iid -> Int4,
        title -> VarChar,
        alt -> VarChar,
        src -> Binary,
        tags -> Array<VarChar>
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, Queryable, Selectable)]
#[diesel(table_name = image_article)]
pub struct ImageArticleTable {
    iid: u64,
    image_id: u64,
    article_id: u64,
    width: Option<f64>,
    height: Option<f64>,
    opacity: Option<f64>,
}

diesel::table! {
    image_article (iid) {
        iid -> Int4,
        image_id -> Int4,
        article_id -> Int4,
        width -> Nullable<Float>,
        height -> Nullable<Float>,
        opacity -> Nullable<Float>,
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, Queryable, Selectable)]
#[diesel(table_name = author)]
pub struct AuthorTable {
    iid: u64,
    uid: u64,
    name: String,
    email: String,
}

diesel::table! {
    author (iid) {
        iid -> Int4,
        uid -> Int4,
        name -> VarChar,
        email -> VarChar,
    }
}
