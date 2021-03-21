use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Filter {
	#[serde(rename = "server-id")]
	is_server_id: bool,
	#[serde(rename = "accept-users")]
	accept_users: Vec<u64>,
	#[serde(rename = "disocrd-server-name-or-id")]
	server_name: String,
}

impl Filter {
	pub fn new(server_id: bool, accept_users: Vec<u64>, server_name: String) -> Self {
		Self { is_server_id: server_id, accept_users, server_name }
	}

	/// Get a reference to the filter's server name.
	pub fn get_server_name(&self) -> &String {
		&self.server_name
	}

	/// Get a reference to the filter's accept users.
	pub fn get_accept_users(&self) -> &Vec<u64> {
		&self.accept_users
	}

	/// Get a reference to the filter's server id.
	pub fn get_is_server_id(&self) -> &bool {
		&self.is_server_id
	}
}
