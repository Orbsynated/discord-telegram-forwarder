use super::{checker::is_discord_message_ok, lib::format_message_to_telegram};
use crate::utils::storage::{CONFIG, TG_CLIENT};
use log::{debug, error, info, log_enabled};
use serenity::{http::CacheHttp, model::{channel::Message, id::GuildId}};
use serenity::{
	async_trait,
	model::{guild::GuildStatus, prelude::Ready},
};
use serenity::{
	client::{Context, EventHandler},
	model::guild::Guild,
};
use tokio::time::{timeout, Duration};

const TIMEOUT: Duration = Duration::from_secs(5);

pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {

	async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
		info!("Forwarder is ready for messages");

		if log_enabled!(log::Level::Debug) {
			print_guild_names(_data_about_bot);
		}
	}


	async fn message(&self, _ctx: Context, _new_message: Message) {
		let _test_guild = _new_message.guild(&_ctx.cache).await.unwrap();
		let conf = timeout(TIMEOUT, CONFIG.get().read()).await.unwrap();
		// Check if message matches the filter in config
		let is_ok = is_discord_message_ok(&conf, &_new_message, &_ctx.cache).await;
		if is_ok {
			let guild_name_map = |guild: &Guild| guild.name.clone();
			let guild_name = _new_message.guild_field(&_ctx.cache, guild_name_map).await;
			if let Some(name) = guild_name {
				let msg = format_message_to_telegram(_new_message, name, conf.time_zone());
				TG_CLIENT.get().send_message_to_subscribers(msg).await;
			} else {
				error!("Could not get discord server name from message id: {}", _new_message.id.0);
			}
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
