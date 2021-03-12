use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "server-id")]
    server_id: bool,
    #[serde(rename = "accept-users")]
    accept_users: Vec<u32>,
    #[serde(rename = "disocrd-server-name-or-id")]
    server_name: String,
}

impl Filter {
    pub fn new(server_id: bool, accept_users: Vec<u32>, server_name: String) -> Self {
        Self {
            server_id,
            accept_users,
            server_name,
        }
    }
}
