use std::sync::Arc;

use log::{debug, warn};
use serenity::{client::Cache, model::{channel::Message}};

use crate::{config::config::Config, filter::filter::Filter};

pub async fn is_discord_message_ok(conf: &Config, _new_message: &Message, cache: &Arc<Cache>) -> bool {
	if let Some(defined_filters) = &conf.filter {
		for filter in defined_filters {
			let should_guild_id = filter.is_server_id;
			let guild_id = filter.server_name.to_lowercase();
			debug!(
				"
				Checking discord message from server id: {}
				",
				_new_message.guild_id.unwrap()
			);

			if is_guild_exist_in_filter(&guild_id, _new_message, &should_guild_id, cache).await
				&& is_user_exist_in_filter(filter, _new_message)
			{
				return true;
			}
		}
	}
	return false;
}

async fn is_guild_exist_in_filter(
	filter_guild_name_or_id: &String,
	msg: &Message,
	is_server_id: &bool,
	cache: &Cache,
) -> bool {
	if *is_server_id {
		if msg.guild_id.unwrap().to_string().eq_ignore_ascii_case(filter_guild_name_or_id) {
			return true;
		}
	} else {
		let guild = msg.guild(cache).await;
		if let Some(guild) = guild {
			if guild.name.to_string().eq_ignore_ascii_case(filter_guild_name_or_id) {
				return true;
			}
		} else {
			warn!("There was an error getting the discord server name matching the filter");
		}
	}
	return false;
}

fn is_user_exist_in_filter(filter: &Filter, msg: &Message) -> bool {
	for user in &filter.accept_users {
		if msg.author.id.eq(user) {
			return true;
		}
	}
	return false;
}
