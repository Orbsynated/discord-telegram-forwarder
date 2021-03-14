use std::{marker::PhantomData};

use log::LevelFilter;
use serde::{de::Visitor, Deserialize, Serialize};
static LOG_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

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
        deserializer.deserialize_enum("VerbosityLevel", &LOG_LEVEL_NAMES, VerbosityLevelVisitor::new())
    }
}

struct VerbosityLevelVisitor {
    marker: PhantomData<fn() -> VerbosityLevel>,
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
        formatter.write_str("Level filter enum")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let t: VerbosityLevel = data.variant().unwrap().0;
        // let test = LOG_LEVEL_NAMES
        //     .iter()
        //     .position(|&name| eq_ignore_ascii_case(name, self.0.as_str()))
        //     .map(|p| VerbosityLevel::from_usize(p).unwrap());

            Ok(VerbosityLevel(LevelFilter::Debug))
    }
}
pub fn eq_ignore_ascii_case(a: &str, b: &str) -> bool {
    fn to_ascii_uppercase(c: u8) -> u8 {
        if c >= b'a' && c <= b'z' {
            c - b'a' + b'A'
        } else {
            c
        }
    }

    if a.len() == b.len() {
        a.bytes()
            .zip(b.bytes())
            .all(|(a, b)| to_ascii_uppercase(a) == to_ascii_uppercase(b))
    } else {
        false
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
