use cli::cli as cli_utils;
use connector::connector::MessageHandler;
use serenity::Client;
use utils::constants::TokenType;
use log::{debug, error, log_enabled, info, Level};
mod config;
mod utils;
mod filter;
mod connector;
mod cli;
mod telegram;


#[tokio::main]
async fn main() {
    let config = cli_utils::init_config();

    let mut client = Client::builder(config.get_token(TokenType::Discord))
        .event_handler(MessageHandler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
