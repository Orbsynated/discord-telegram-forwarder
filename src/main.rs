use std::sync::Arc;

use cli::cli as cli_utils;
use connector::connector::MessageHandler;
use log::{error, info};
use serenity::{
	client::{bridge::gateway::GatewayIntents, EventHandler},
	Client,
};
use telegram::telegram::TelegramBot;
use utils::{
	constants::TokenType,
	storage::{CONFIG, TG_CLIENT},
};
mod cli;
use tokio::sync::RwLock;
mod config;
mod connector;
mod filter;
mod telegram;
mod utils;
use utils::extensions::ErrorExtensions;

const MODULE_PATH: &str = module_path!();
#[tokio::main]
async fn main() {
	info!("Starting discord event forwarder");
	let config = RwLock::new(cli_utils::init_config(MODULE_PATH).expect_with_log("Error parsing config"));
	CONFIG.set(config);

	let conf = CONFIG.get().read().await;

	let discord_token = conf.get_token(TokenType::Discord).to_owned();

	let telegram_token = conf.get_token(TokenType::Telegram).to_owned();

	let telegram_client = TelegramBot::init_telegram_bot(telegram_token);

	TG_CLIENT.set(Arc::new(telegram_client));

	tokio::task::spawn(async {
		TG_CLIENT.get().listen_to_subscriptions().await;
	});

	let mut client = create_client(discord_token, MessageHandler).await.expect("Error creating discord client");

	if let Err(why) = client.start().await {
		error!("An error occurred while running the client: {:?}", why);
	}
}

async fn create_client<H: EventHandler + 'static>(
	token: String,
	event_handler: H,
) -> Result<Client, Box<dyn std::error::Error>> {
	let client: Client =
		Client::builder(token).intents(GatewayIntents::GUILD_MESSAGES).event_handler(event_handler).await?;
	Ok(client)
}
