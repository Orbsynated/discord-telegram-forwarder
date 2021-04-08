use log::debug;
use serenity::model::channel::Message;

use crate::config::config::Config;

/// Checks the discord message according to the config
/// Returns the discord server name
pub async fn is_discord_message_ok(conf: &Config, _new_message: &Message) -> Option<usize> {
	if let Some(defined_filters) = conf.get_filter() {
		for (index, filter) in defined_filters.iter().enumerate() {
			let guild_id = filter.get_server_id();
			let guild_accept_users = filter.get_accept_users();

			debug!("Checking discord message from server id: {}", _new_message.guild_id.unwrap());

			if is_guild_exist_in_filter(&guild_id, _new_message).await
				&& is_user_exist_in_filter(guild_accept_users, _new_message)
			{
				return Some(index);
			}
		}
	}
	None
}

pub fn get_server_name(guild_id: String, conf: &Config, filter_index: usize) -> String {
	let filter = &conf.get_filter().as_ref().unwrap()[filter_index];

	// Get server name from the config by the id of the message
	if let Some(servers) = conf.get_servers() {
		let find_map_fn = |(server_id, server_name): &(u64, String)| {
			if filter.get_server_id().eq(server_id) {
				return Some(server_name.clone());
			}
			None
		};

		if let Some(server_name) = servers.iter().find_map(find_map_fn) {
			return server_name;
		}
	}
	// If there is no filter or the server id does not match the message's server id, then return the server id numberQ
	return guild_id.to_string();
}

async fn is_guild_exist_in_filter(filter_guild_name_or_id: &u64, msg: &Message) -> bool {
	if msg.guild_id.unwrap().eq(filter_guild_name_or_id) {
		return true;
	}
	return false;
}

fn is_user_exist_in_filter(accept_users: &Vec<u64>, msg: &Message) -> bool {
	for user in accept_users {
		if msg.author.id.eq(user) {
			return true;
		}
	}
	return false;
}
