use serde::{Deserialize, Serialize};
use std::fs::File;
// use log::{debug, error, log_enabled, info};

use crate::{filter::filter::Filter, utils::constants::TokenType};

use super::utils::VerbosityLevel;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "discord-token")]
    discord_token: String,
    #[serde(rename = "telegram-bot-token")]
    telegram_token: String,

    filter: Option<Vec<Filter>>,

    #[serde(rename = "verbosity-level")]
    verbosity_level: VerbosityLevel,

    #[serde(skip_serializing)]
    config_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_path: Some(String::default()),
            discord_token: String::default(),
            telegram_token: String::default(),
            filter: None,
            verbosity_level: VerbosityLevel(log::LevelFilter::Debug),
        }
    }
}

impl Config {
    pub fn new(
        discord_token: String,
        telegram_token: String,
        filter: Option<Vec<Filter>>,
        verbosity_level: VerbosityLevel,
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

    pub fn get_token(&self, token_type: TokenType) -> &String {
        match token_type {
            TokenType::Discord => &self.discord_token,
            TokenType::Telegram => &self.telegram_token,
        }
    }

    /// Get a reference to the config's filter.
    pub fn get_filter(&self) -> &Option<Vec<Filter>> {
        &self.filter
    }
}
