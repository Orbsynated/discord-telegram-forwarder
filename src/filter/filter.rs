use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Filter {
	#[serde(rename = "accept-users", default)]
	accept_users: Vec<u64>,
	#[serde(rename = "discord-server-id")]
	server_id: u64,
}

impl Filter {
	/// Get a reference to the filter's accept users.
	pub fn get_accept_users(&self) -> &Vec<u64> {
		&self.accept_users
	}

	/// Get a reference to the filter's server id.
	pub fn get_server_id(&self) -> &u64 {
		&self.server_id
	}
}
