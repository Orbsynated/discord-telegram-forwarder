use crate::{config::config::Config, utils::constants};
use clap::{crate_authors, crate_version, App, Arg};
use constants::DEFAULT_LEVEL;
use log::LevelFilter;

fn setup_args() -> [Arg<'static, 'static>; 4] {
    let config_arg = Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE")
        .help("Sets the config file")
        .takes_value(true);

    let debug_arg = Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Turn debugging information on")
        .multiple(true);

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

pub fn init_config() -> Config {
    let level: LevelFilter;
    let config: Config;
    let args: [Arg; 4] = setup_args();
    let matches = App::new("Discord Telegram Forwarder")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("This App Forwards all message in discord to telegram, with filter options.")
        .args(&args)
        .get_matches();

    if let Some(config_path) = matches.value_of("config") {
        config = Config::load_config(config_path).unwrap();
        level = config.verbosity_level().to_owned();
    } else {
        let is_debug = matches.is_present("debug");
        let discord_token = matches.value_of(constants::DISCORD_TOKEN_NAME).unwrap();
        let telegram_token = matches.value_of(constants::DISCORD_TOKEN_NAME).unwrap();
        level = match is_debug {
            true => LevelFilter::Debug,
            _ => DEFAULT_LEVEL,
        };

        config = Config::new(
            Some(String::from(discord_token)),
            Some(String::from(telegram_token)),
            None,
            DEFAULT_LEVEL,
            None,
        )
    }
    env_logger::builder().filter_level(level).init();
    config
}
