use super::{
	checker::{get_server_name, is_discord_message_ok},
	lib::format_message_to_telegram,
};
use crate::utils::storage::{CONFIG, TG_CLIENT};
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use tokio::time::{timeout, Duration};

const TIMEOUT: Duration = Duration::from_secs(5);

pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
	async fn message(&self, _ctx: Context, _new_message: Message) {
		let conf = timeout(TIMEOUT, CONFIG.get().read()).await.unwrap();
		// Check if message matches the filter in config
		if let Some(filter_index) = is_discord_message_ok(&conf, &_new_message).await {
			let server_name = get_server_name(_new_message.guild_id.unwrap().to_string(), &conf, filter_index);
			let msg = format_message_to_telegram(_new_message, &server_name, conf.get_time_zone());
			TG_CLIENT.get().send_message_to_subscribers(msg).await;
		}
	}
}
