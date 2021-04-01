use tokio::sync::RwLock;

use futures::StreamExt;
use telegram_bot::{Api, CanSendMessage, ChatId, MessageKind, MessageOrChannelPost, UpdateKind};

pub struct TelegramBot {
	pub api: Api,
	chat_ids: RwLock<Vec<i64>>,
}

impl TelegramBot {
	pub fn init_telegram_bot(token: String) -> Self {
		let tg_client = Api::new(&token);
		Self { api: tg_client, chat_ids: RwLock::new(Vec::new()) }
	}

	/// Listen to telegram subscriptions by using the "/subscribe" or "subscribe" command in telegram
	pub async fn listen_to_subscriptions(&self) {
		let mut stream = self.api.stream();
		while let Some(update) = stream.next().await {
			if let Ok(update) = update {
				if let UpdateKind::Message(message) = update.kind {
					if let MessageKind::Text { ref data, .. } = message.kind {
						if data == "subscribe" || data == "/subscribe" {
							let chat_id = message.chat.id().to_string().parse::<i64>().unwrap();
							// Add to subscription list only if is not already included
							if !self.chat_ids.read().await.iter().any(|&i| i == chat_id) {
								self.chat_ids.write().await.push(chat_id);
							}
						}
					}
				}
			}
		}
	}

	pub async fn send_message_to_subscribers(&self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
		for id in self.chat_ids.read().await.iter() {
			self.send_message_to(msg, id).await?;
		}
		Ok(())
	}

	async fn send_message_to(&self, msg: &str, chat_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
		let chat = ChatId::new(chat_id.clone());
		self.api.send(chat.text(msg)).await?;
		Ok(())
	}
}
