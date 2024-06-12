use anyhow::Result;
use chrono::{Local, NaiveDateTime, TimeZone};
use core::fmt::Display;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::consts::DATE_FORMAT;

/// Semver
#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize, ToSchema)]
pub struct Version {
    #[schema(example = "4")]
    major: usize,
    #[schema(example = "2")]
    minor: usize,
    #[schema(example = "0")]
    patch: usize,
}

impl Version {
    #[allow(dead_code)]
    /**
    Constructs a new Version
    */
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Runtime of a service
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, ToSchema)]
pub struct Runtime {
    /// Date the service started
    #[schema(example = "11.03.24 [16:34]")]
    date: String,
    /// Time since epoch, in milliseconds
    #[schema(example = "69420")]
    time: usize,
}

impl Runtime {
    #[allow(dead_code)]
    /// Constructs a new Runtime
    pub fn new(date: String, time: usize) -> Self {
        Self { date, time }
    }

    pub fn init() -> Self {
        Self {
            date: format!("{}", chrono::Local::now().format(DATE_FORMAT)),
            time: Local::now().timestamp_millis() as usize,
        }
    }

    /// Updates the time-field
    pub fn update_time(&self) -> Result<Self> {
        Ok(Self {
            date: self.date.clone(),
            time: Local::now().timestamp_millis() as usize - self.time,
        })
    }
}
