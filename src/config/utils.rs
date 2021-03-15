use std::marker::PhantomData;

use log::LevelFilter;
use log::{debug, error, info, log_enabled, Level};
use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};

const LOG_LEVEL_NAMES: [&'static str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];
const DEFAULT_LEVEL: LevelFilter = LevelFilter::Info;

#[derive(Debug, PartialEq)]
pub struct VerbosityLevel(pub LevelFilter);

impl Serialize for VerbosityLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for VerbosityLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(VerbosityLevelVisitor::new())
    }
}
///
/// We use a different struct as a visitor and not VerbosityLevel because we don't want to create a new verbosity level struct every time
///  We deserialize (even though we drop it right after)
struct VerbosityLevelVisitor {
    // Using PhantomData so the compiler thinks VerbosityLevelVisitor owns VerbosityLevel but will actually always be empty
    marker: PhantomData<VerbosityLevel>,
}

impl VerbosityLevelVisitor {
    fn new() -> Self {
        VerbosityLevelVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for VerbosityLevelVisitor {
    type Value = VerbosityLevel;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A string containing verbosity level")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match LOG_LEVEL_NAMES
            .iter()
            .position(|&name| name.eq_ignore_ascii_case(v))
            // Set default verbosity level as INFO
            .map(|p| VerbosityLevel::from_usize(p))
            .unwrap_or(None)
        {
            Some(value) => Ok(VerbosityLevel(value)),
            None => Err(Error::custom(
                "Something went wrong with de-serializing verbosity level",
            )),
        }
    }
}

impl VerbosityLevel {
    fn from_usize(u: usize) -> Option<LevelFilter> {
        match u {
            0 => Some(LevelFilter::Off),
            1 => Some(LevelFilter::Error),
            2 => Some(LevelFilter::Warn),
            3 => Some(LevelFilter::Info),
            4 => Some(LevelFilter::Debug),
            5 => Some(LevelFilter::Trace),
            _ => None,
        }
    }
}
