use cli::cli as cli_utils;
use connector::connector::MessageHandler;
use log::{debug, error};
use serenity::{
	client::{bridge::gateway::GatewayIntents, EventHandler},
	Client,
};
use telegram::telegram::TelegramBot;
use utils::{constants::TokenType, storage::CONFIG};
mod cli;
use tokio::sync::RwLock;
mod config;
mod connector;
mod filter;
mod telegram;
mod utils;
use utils::extensions::ErrorExtensions;

#[tokio::main]
async fn main() {
	debug!("Starting discord event forwarder");
	let module_path = module_path!().to_string();
	let config = RwLock::new(cli_utils::init_config(module_path).expect_with_log("Error parsing config"));
	CONFIG.set(config);

	let conf = CONFIG.get().read().await;

	let discord_token = conf.get_token(TokenType::Discord).to_owned();

	let telegram_token = conf.get_token(TokenType::Telegram).to_owned();

	let mut telegram_client = TelegramBot::init_telegram_bot(telegram_token);

	tokio::task::spawn(async move {
		telegram_client.listen_to_subscriptions().await;
	});

	let mut client = create_client(discord_token, MessageHandler).await.expect("Error creating client");

	if let Err(why) = client.start().await {
		error!("An error occurred while running the client: {:?}", why);
		panic!()
	}
}

async fn create_client<H: EventHandler + 'static>(
	token: String,
	event_handler: H,
) -> Result<Client, Box<dyn std::error::Error>> {
	let client: Client =
		Client::builder(token).intents(GatewayIntents::GUILD_MESSAGES).event_handler(event_handler).await.unwrap();
	Ok(client)
}
