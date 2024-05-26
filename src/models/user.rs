use anyhow::Result;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthUser {
    #[schema(example = "nilsmf")]
    id: String,
    /// Hashed password
    pwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Internal ID, used in databases
    iid: usize,
    /// User identity
    id: String,
    /// Verified emailtype = "array", items(type = "integer", format = "int32")
    email: Email,
}

#[derive(Debug, Serialize, Deserialize)]
/// Email struct, an email is verified to be existing and valid.
pub struct Email(String);

impl Email {
    /// Creates a new email
    ///
    /// # Error
    /// If the given email is invalid, it will return an error.
    ///
    /// If the given email is not validated within a timelimit, it will error.
    #[allow(dead_code)]
    pub async fn new(_email: &str) -> Result<Self> {
        unimplemented!()
    }
}
