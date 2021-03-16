use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{fs::File};
// use log::{debug, error, log_enabled, info};

use crate::{
    filter::filter::Filter,
    utils::constants::{TokenType, DEFAULT_LEVEL},
};

use super::lib::{deserialize_level_filter, serialize_level_filter};
use crate::utils::constants::DEFAULT_LEVEL_FN;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "discord-token")]
    discord_token: Option<String>,
    #[serde(rename = "telegram-bot-token")]
    telegram_token: Option<String>,

    filter: Option<Vec<Filter>>,

    #[serde(
        rename = "verbosity-level",
        serialize_with = "serialize_level_filter",
        deserialize_with = "deserialize_level_filter",
        default = "DEFAULT_LEVEL_FN"
    )]
    verbosity_level: LevelFilter,

    #[serde(skip_serializing)]
    config_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_path: None,
            discord_token: None,
            telegram_token: None,
            filter: None,
            verbosity_level: DEFAULT_LEVEL,
        }
    }
}

impl Config {
    pub fn new(
        discord_token: Option<String>,
        telegram_token: Option<String>,
        filter: Option<Vec<Filter>>,
        verbosity_level: LevelFilter,
        config_path: Option<String>,
    ) -> Self {
        Self {
            discord_token,
            telegram_token,
            filter,
            verbosity_level,
            config_path,
        }
    }

    pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let yaml: Config = serde_yaml::from_reader(file)?;
        Ok(yaml)
    }

    pub fn get_token(&self, token_type: TokenType) -> &Option<String> {
        match token_type {
            TokenType::Discord => &self.discord_token,
            TokenType::Telegram => &self.telegram_token,
        }
    }

    /// Get a reference to the config's filter.
    pub fn get_filter(&self) -> &Option<Vec<Filter>> {
        &self.filter
    }

    /// Get a reference to the config's verbosity level.
    pub fn verbosity_level(&self) -> &LevelFilter {
        &self.verbosity_level
    }
}
