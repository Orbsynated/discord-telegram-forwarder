use std::fs::File;

use self::filter::Filter;
use serde::{Deserialize, Serialize};

#[path = "../filter/filter.rs"]
mod filter;
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
        let yaml = serde_yaml::from_reader(file)?;
        Ok(yaml)
    }
}
