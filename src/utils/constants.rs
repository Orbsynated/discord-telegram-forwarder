use log::LevelFilter;

// Command Line Constants
pub const DISCORD_TOKEN_NAME: &str = "discord.token";
pub const TELEGRAM_TOKEN_NAME: &str = "telegram.token";

pub const DEFAULT_LEVEL: LevelFilter = log::LevelFilter::Info;
pub const DEFAULT_LEVEL_FN: fn() -> LevelFilter = || DEFAULT_LEVEL;

pub enum TokenType {
    Discord,
    Telegram,
}
