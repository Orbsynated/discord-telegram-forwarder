use std::{rc::Rc, sync::{Arc, Mutex, RwLock}};

use cli::cli as cli_utils;
use config::config::Config;
use connector::connector::MessageHandler;
use serenity::Client;
use state::Storage;
use utils::constants::TokenType;
use log::{debug, error};
mod config;
mod utils;
mod filter;
mod connector;
mod cli;
mod telegram;

static CONFIG: Storage<Arc<RwLock<Config>>> = Storage::new();

#[tokio::main]
async fn main() {
    debug!("Starting discord event forwarder");
    let config = || Arc::new(RwLock::new(cli_utils::init_config()));
    CONFIG.set(config());
    
    let discord_token = CONFIG.get().read().expect("Unexpected error!");

    let mut client = Client::builder(discord_token.get_token(TokenType::Discord).to_owned().unwrap_or(String::default()))
        .event_handler(MessageHandler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
