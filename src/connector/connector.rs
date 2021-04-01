use super::checker::is_discord_message_ok;
use crate::utils::storage::{CONFIG, TG_CLIENT};
use log::{debug, info, log_enabled};
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::{
	async_trait,
	model::{guild::GuildStatus, prelude::Ready},
};
use tokio::{
	task::spawn,
	time::{timeout, Duration},
};

const TIMEOUT: Duration = Duration::from_secs(5);



pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
	async fn message(&self, _ctx: Context, _new_message: Message) {
		// Create an async task to send telegram message to not block current thread
		spawn(async {
			let conf = timeout(TIMEOUT, CONFIG.get().read()).await.unwrap();
			let is_ok = is_discord_message_ok(conf.clone(), _new_message, _ctx.cache).await;
			if is_ok {
				// TG_CLIENT.get().send_message_to_subscribers(_new_message)
				//TODO: forward
			}
		});
	}

	async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
		info!("Forwarder is ready for messages");

		if log_enabled!(log::Level::Debug) {
			print_guild_names(_data_about_bot);
		}
	}
}

impl MessageHandler {
	
}
fn print_guild_names(_data_about_bot: Ready) {
	let guild_names: Vec<String> = _data_about_bot
		.guilds
		.iter()
		.filter_map(|guild| {
			if let GuildStatus::OnlineGuild(online) = guild {
				return Some(online.name.clone());
			}
			return None;
		})
		.collect();
	debug!("Currently watching these servers: {:#?}", guild_names)
}
