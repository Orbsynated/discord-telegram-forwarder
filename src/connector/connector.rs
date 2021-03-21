use log::{debug, info, log_enabled, warn};
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::{
	async_trait,
	model::{guild::GuildStatus, prelude::Ready},
};

use super::checker::is_discord_message_ok;
use crate::utils::storage::CONFIG;
use tokio::time::timeout;

pub struct MessageHandler;
#[async_trait]
impl EventHandler for MessageHandler {
	async fn message(&self, _ctx: Context, _new_message: Message) {
		const TIMEOUT: tokio::time::Duration = tokio::time::Duration::from_secs(5);
		match timeout(TIMEOUT, CONFIG.get().read()).await {
			Ok(conf) => {
				let is_ok = is_discord_message_ok(&conf, &_new_message, &_ctx.cache).await;
				if let Ok(ok) = is_ok {
					//TODO: forward
				}
			}
			Err(why) => warn!(
				"Failed to read configuration, cannot know which messages to read so every message will be dropped \n
            {:?}",
				why
			),
		};
	}

	async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
		info!("Forwarder is ready for messages");

		if log_enabled!(log::Level::Debug) {
			print_guild_names(_data_about_bot);
		}
	}
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
