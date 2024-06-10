use crate::models::meta::{Runtime, Version};
use anyhow::{Context, Result};
use utoipa::OpenApi;

/// Contains models used in nilsmf services
pub mod models;

/// Contains utility functions and constants used in nilsmf services
pub mod utils;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "IGNORE")
    ),
)]
#[rocket::get("/")]
fn empty() {}

#[derive(OpenApi)]
#[openapi(
    info(description = "nilsmf-lib: Component Specification"),
    paths(empty),
    components(schemas(Version, Runtime))
)]
struct ApiDoc;

pub fn generate_specification() -> Result<String> {
    serde_yaml::to_string(
        &serde_yaml::from_str::<serde_json::Value>(
            &ApiDoc::openapi()
                .to_json()
                .context("Failed generating specification")?,
        )
        .context("Failed serializing serde_json::Value to serde_yaml::Value")?,
    )
    .context("Failed deserializing serde_yaml::Value to String")
}
