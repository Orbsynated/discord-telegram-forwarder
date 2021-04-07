use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Filter {
	#[serde(rename = "accept-users", default)]
	pub accept_users: Vec<u64>,
	#[serde(rename = "discord-server-id")]
	pub server_id: u64,
	#[serde(rename = "discord-server-name", default)]
	pub server_name: String,
}

