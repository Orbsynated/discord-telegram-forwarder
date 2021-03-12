#[path = "utils/constants.rs"]
mod constants;
#[path = "./config/config.rs"]
mod config;
#[path = "filter/filter.rs"]
mod filter;

use clap::{crate_authors, crate_version, App, Arg};
use config::Config;
use constants::cl;

fn main() {
    let args = init();
}

fn init() -> Config {
    let args = setup_args();
    let matches = App::new("Discord Telegram Forwarder")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("This App Forwards all message in discord to telegram, with filter options.")
        .args(&args)
        .get_matches();

    if let Some(config_path) = matches.value_of("config") {
        Config::load_config(config_path).unwrap()
    } else {
        let is_debug = matches.is_present("debug");
        let discord_token = matches.value_of(cl::DISCORD_TOKEN_NAME).unwrap();
        let telegram_token = matches.value_of(cl::DISCORD_TOKEN_NAME).unwrap();

        Config::new(
            String::from(discord_token),
            String::from(telegram_token),
            None,
            is_debug,
            None,
        )
    }
}

fn setup_args() -> Vec<Arg<'static, 'static>> {
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

    let discord_token = Arg::with_name(cl::DISCORD_TOKEN_NAME)
        .long(cl::DISCORD_TOKEN_NAME)
        .value_name("TOKEN")
        .help("Discord token")
        .takes_value(true)
        .group("secrets");
    let telegram_token = Arg::with_name(cl::TELEGRAM_TOKEN_NAME)
        .long(cl::TELEGRAM_TOKEN_NAME)
        .value_name("TOKEN")
        .help("Telegram token")
        .takes_value(true)
        .group("secrets");

    vec![config_arg, debug_arg, discord_token, telegram_token]
}
