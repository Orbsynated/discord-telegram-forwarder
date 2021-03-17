use std::sync::RwLock;
use cli::cli as cli_utils;
use connector::connector::MessageHandler;
use log::{debug, error};
use serenity::Client;
use utils::{constants::TokenType, storage::CONFIG};
mod cli;
mod config;
mod connector;
mod filter;
mod telegram;
mod utils;
use utils::extensions::ErrorExtensions;

#[tokio::main]
async fn main() {
    debug!("Starting discord event forwarder");
    let config = RwLock::new(cli_utils::init_config().expect_with_log("Error parsing config"));
    CONFIG.set(config);

    let discord_token = CONFIG.get().read().unwrap();

    let mut client = Client::builder(discord_token.get_token(TokenType::Discord).to_owned())
        .event_handler(MessageHandler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
