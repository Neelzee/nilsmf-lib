use core::fmt::Display;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Semver
#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize, ToSchema)]
pub struct Version {
    major: usize,
    minor: usize,
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
    date: String,
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
            date: format!("{}", chrono::Local::now().format("%d.%m.%Y [%H:%M]")),
            time: 0,
        }
    }
}
