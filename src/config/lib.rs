use std::marker::PhantomData;

use super::{debug, LevelFilter};
use serde::{
    de::{Error, Visitor},
    Deserializer, Serializer,
};
const LOG_LEVEL_NAMES: [&'static str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

pub fn deserialize_level_filter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(LevelFilterVisitor::new())
}

pub fn serialize_level_filter<S>(level: &LevelFilter, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match serializer.serialize_str(level.as_str()) {
        Ok(output) => Ok(output),
        Err(de_error) => Err(de_error),
    }
}
/// We use a different struct as a visitor and not LevelFilter because we don't want to create a new verbosity level struct every time
/// We deserialize (even though we drop it right after)
struct LevelFilterVisitor {
    // Using PhantomData so the compiler thinks LevelFilterVisitor owns LevelFilter but will actually always be empty
    marker: PhantomData<LevelFilter>,
}

impl LevelFilterVisitor {
    fn new() -> Self {
        LevelFilterVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for LevelFilterVisitor {
    type Value = LevelFilter;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A string containing Level Filter enum value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        debug!("Trying to de-serialize level filter");
        match LOG_LEVEL_NAMES
            .iter()
            .position(|&name| name.eq_ignore_ascii_case(value))
            // Set default verbosity level as INFO
            .map(|p| level_filter_from_usize(p))
            .unwrap_or(None)
        {
            Some(value) => {
                Ok(value)
            },
            None => Err(Error::custom(format!("Something went wrong with de-serializing verbosity level, received {}, expected level filter enum value", value)
            )),
        }
    }
}
fn level_filter_from_usize(u: usize) -> Option<LevelFilter> {
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
