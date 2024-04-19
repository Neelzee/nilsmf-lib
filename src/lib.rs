use serde::{Deserialize, Serialize};

mod models;

#[derive(Serialize, Deserialize)]
pub struct Wrapper<T> {
    iid: String,
    content: T,
}
