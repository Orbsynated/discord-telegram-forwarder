use log::{debug, info, log_enabled};
use serenity::{client::{Context, EventHandler, RawEventHandler}, collector::MessageFilter};
use serenity::model::channel::Message;
use serenity::{
    async_trait,
    model::{guild::GuildStatus, prelude::Ready},
};

pub struct MessageHandler;

impl RawEventHandler for MessageHandler {}

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, _ctx: Context, _new_message: Message) {

    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        info!("Forwarder is ready for messages");

        if log_enabled!(log::Level::Debug) {
            print_guild_names(_data_about_bot);
        }
    }
}

fn print_guild_names(_data_about_bot: Ready) {
    let guild_names: Vec<String> = _data_about_bot
        .guilds
        .iter()
        .filter_map(|guild| {
            if let GuildStatus::OnlineGuild(online) = guild {
                return Some(online.name.clone());
            }
            return None;
        })
        .collect();
    debug!("Currently watching these servers: {:#?}", guild_names)
}
