use crate::utils::extensions::ErrorExtensions;
use crate::{config::config::Config, utils::constants};
use chrono_tz::Tz;
use clap::{crate_authors, crate_version, App, Arg};
use constants::{DEFAULT_CONFIG_FILE_NAME, DEFAULT_LEVEL};
use log::LevelFilter;
use std::env::current_dir;

fn setup_args() -> [Arg<'static, 'static>; 4] {
	let config_arg = Arg::with_name("config")
		.short("c")
		.long("config")
		.value_name("FILE")
		.help("Sets the config file")
		.takes_value(true);

	let debug_arg =
		Arg::with_name("debug").short("d").long("debug").help("Turn debugging information on").multiple(true);

	let discord_token = Arg::with_name(constants::DISCORD_TOKEN_NAME)
		.long(constants::DISCORD_TOKEN_NAME)
		.value_name("TOKEN")
		.help("Discord token")
		.takes_value(true)
		.group("secrets");
	let telegram_token = Arg::with_name(constants::TELEGRAM_TOKEN_NAME)
		.long(constants::TELEGRAM_TOKEN_NAME)
		.value_name("TOKEN")
		.help("Telegram token")
		.takes_value(true)
		.group("secrets");

	[config_arg, debug_arg, discord_token, telegram_token]
}

/// Initialize config using yaml file or command line arguments
///
/// `main_module_path` is the main module path which will is used to show only logs from this namespace
pub fn init_config(main_module_path: &'static str) -> Result<Config, Box<dyn std::error::Error>> {
	let level: LevelFilter;
	let config: Config;
	let args: [Arg; 4] = setup_args();
	let matches = App::new("Discord Telegram Forwarder")
		.version(crate_version!())
		.author(crate_authors!(",\n"))
		.about("This App Forwards all message in discord to telegram, with filter options.")
		.args(&args)
		.get_matches();

	// If config path is already in the arguments, load it
	if let Some(config_path) = matches.value_of("config") {
		// Config path is defined in arguments
		config = Config::try_load_config(config_path).expect_with_log("Error parsing yaml config");
	} else {
		let default_path = current_dir().unwrap().join(DEFAULT_CONFIG_FILE_NAME);

		// Try to get config.yaml from executable folder
		if let Ok(conf) = Config::try_load_config(default_path.to_str().unwrap()) {
			config = conf;
		} else {
			let is_debug = matches.is_present("debug");
			let discord_token = matches.value_of(constants::DISCORD_TOKEN_NAME).expect("Missing Discord Token");
			let telegram_token = matches.value_of(constants::DISCORD_TOKEN_NAME).expect("Missing Telegram Token");
			level = match is_debug {
				true => LevelFilter::Debug,
				_ => DEFAULT_LEVEL,
			};

			config = Config::new(String::from(discord_token), String::from(telegram_token), None, level, Tz::UTC)
		}
	}
	env_logger::builder().filter_module(&main_module_path, config.verbosity_level().clone()).init();

	Ok(config)
}
