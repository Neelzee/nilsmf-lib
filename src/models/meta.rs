use core::fmt::Display;
use serde::{Deserialize, Serialize};
use time::Date;

/// Semver
#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

impl Version {
    #[allow(dead_code)]
    /// Constucts a new Version
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

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// Runtime of a service
pub struct Runtime {
    date: Date,
    time: usize,
}

impl Runtime {
    #[allow(dead_code)]
    /// Constructs a new Runtime
    pub fn new(date: Date, time: usize) -> Self {
        Self { date, time }
    }
}
