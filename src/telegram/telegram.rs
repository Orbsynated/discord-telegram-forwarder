use telegram_bot::{Api, CanSendMessage, ChatId};

struct TelegramBot {
    token: String,
    api: Api,
    chat_id: Option<i64>
}

impl TelegramBot {
    pub fn init_telegram_bot(token: String) -> Self {
        let tg_client = Api::new(&token);
        Self {
            api: tg_client,
            token,
            chat_id: None,
        }
    }


    pub async fn send_message_to(&self, msg: String, chat_id: i64) ->  Result<(), Box<dyn std::error::Error>>{
        let chat = ChatId::new(chat_id);
        if let Ok(testing) = self.api.send(chat.text(msg)).await{
        }
        Ok(())
    }
}
