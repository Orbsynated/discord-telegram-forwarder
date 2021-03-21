use cli::cli as cli_utils;
use connector::connector::MessageHandler;
use log::{debug, error};
use serenity::{
	client::{bridge::gateway::GatewayIntents, EventHandler},
	Client,
};
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
	let config = RwLock::new(cli_utils::init_config().expect_with_log("Error parsing config"));
	CONFIG.set(config);

	let discord_token = CONFIG.get().read().await.get_token(TokenType::Discord).to_owned();

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
