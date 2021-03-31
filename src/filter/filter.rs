use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Filter {
	#[serde(rename = "server-id")]
	pub is_server_id: bool,
	#[serde(rename = "accept-users")]
	pub accept_users: Vec<u64>,
	#[serde(rename = "disocrd-server-name-or-id")]
	pub server_name: String,
}

