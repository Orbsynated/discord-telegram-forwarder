use log::debug;
use serenity::model::channel::Message;

use crate::{config::config::Config, filter::filter::Filter};

pub async fn is_discord_message_ok<'a, 'b>(conf: &'a Config, _new_message: &'b Message) -> Option<&'a String> {
	if let Some(defined_filters) = &conf.filter {
		for filter in defined_filters {
			let guild_id = filter.server_id;
			debug!(
				"
				Checking discord message from server id: {}
				",
				_new_message.guild_id.unwrap()
			);

			if is_guild_exist_in_filter(&guild_id, _new_message).await && is_user_exist_in_filter(filter, _new_message)
			{
				return Some(&filter.server_name);
			}
		}
	}
	return None;
}

async fn is_guild_exist_in_filter(filter_guild_name_or_id: &u64, msg: &Message) -> bool {
	if msg.guild_id.unwrap().eq(filter_guild_name_or_id) {
		return true;
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
