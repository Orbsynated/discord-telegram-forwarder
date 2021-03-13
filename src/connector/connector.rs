use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;

pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, _ctx: Context, _new_message: Message) {
        
    }
}
