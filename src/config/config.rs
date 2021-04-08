use chrono_tz::Tz;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use serenity::client::validate_token;
use std::fs::File;
// use log::{debug, error, log_enabled, info};

use crate::{
	filter::filter::Filter,
	utils::constants::{TokenType, DEFAULT_LEVEL},
};

use super::lib::{deserialize_level_filter, deserialize_time_zone, serialize_level_filter, serialize_time_zone};
use crate::utils::constants::{DEFAULT_LEVEL_FN, DEFAULT_TIMEZONE_FN};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
	#[serde(rename = "discord-token", default)]
	discord_token: String,
	#[serde(rename = "telegram-bot-token", default)]
	telegram_token: String,

	#[serde(default)]
	filter: Option<Vec<Filter>>,

	#[serde(
		rename = "verbosity-level",
		serialize_with = "serialize_level_filter",
		deserialize_with = "deserialize_level_filter",
		default = "DEFAULT_LEVEL_FN"
	)]
	verbosity_level: LevelFilter,

	#[serde(
		rename = "time-zone",
		serialize_with = "serialize_time_zone",
		deserialize_with = "deserialize_time_zone",
		default = "DEFAULT_TIMEZONE_FN"
	)]
	time_zone: Tz,

	#[serde(default)]
	servers: Option<Vec<(u64, String)>>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			discord_token: String::default(),
			telegram_token: String::default(),
			filter: None,
			verbosity_level: DEFAULT_LEVEL,
			time_zone: Tz::UTC,
			servers: None,
		}
	}
}

impl Config {
	pub fn new(
		discord_token: String,
		telegram_token: String,
		filter: Option<Vec<Filter>>,
		verbosity_level: LevelFilter,
		time_zone: Tz,
		servers: Option<Vec<(u64, String)>>,
	) -> Self {
		Self { discord_token, telegram_token, filter, verbosity_level, time_zone, servers }
	}

	pub fn try_load_config(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let file = File::open(path)?;
		let yaml: Config = serde_yaml::from_reader(file)?;
		// Will panic if discord token is invalid
		yaml.validate_discord_token();
		Ok(yaml)
	}

	pub fn get_token(&self, token_type: TokenType) -> &String {
		match token_type {
			TokenType::Discord => &self.discord_token,
			TokenType::Telegram => &self.telegram_token,
		}
	}

	fn validate_discord_token(&self) {
		let res = validate_token(self.get_token(TokenType::Discord));
		if let Err(_) = res {
			panic!("Invalid discord token provided");
		}
	}

	/// Get a reference to the config's verbosity level.
	pub fn get_verbosity_level(&self) -> &LevelFilter {
		&self.verbosity_level
	}

	/// Get a reference to the config's time zone.
	pub fn get_time_zone(&self) -> &Tz {
		&self.time_zone
	}

	/// Get a reference to the config's filter.
	pub fn get_filter(&self) -> &Option<Vec<Filter>> {
		&self.filter
	}

	/// Get a reference to the config's servers.
	pub fn get_servers(&self) -> &Option<Vec<(u64, String)>> {
		&self.servers
	}
}
