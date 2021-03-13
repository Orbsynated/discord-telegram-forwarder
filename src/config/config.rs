use serde::{Deserialize, Serialize};
use std::fs::File;

use crate::{filter::filter::Filter, utils::constants::TokenType};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "discord-token")]
    discord_token: String,
    #[serde(rename = "telegram-bot-token")]
    telegram_token: String,
    filter: Option<Vec<Filter>>,

    debug: bool,

    #[serde(skip_serializing)]
    config_path: Option<String>,
}

impl Config {
    pub fn new(
        discord_token: String,
        telegram_token: String,
        filter: Option<Vec<Filter>>,
        debug: bool,
        config_path: Option<String>,
    ) -> Self {
        Self {
            discord_token,
            telegram_token,
            filter,
            debug,
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
