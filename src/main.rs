#[path = "./config/config.rs"]
mod config;
#[path = "filter/filter.rs"]
mod filter;

use config::Config;
use clap::{App, Arg, crate_authors, crate_version};

fn main() {
    
    let matches = App::new("Discord Telegram Forwarder")
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .about("This App Forwards all message in discord to telegram, with filter options.")
    .arg(Arg::from_usage("-c, --config=[FILE] 'Sets the config file"))
    .arg(Arg::from_usage("-d..., --debug... 'Turn debugging information on'"))
    .get_matches();

    // let file_name = "E:\\DEV\\discord-telegram-forwarder\\src\\example.config.yaml";
    // let _file = Config::load_config(file_name).unwrap();
    // print!("{:?}", _file);

}
