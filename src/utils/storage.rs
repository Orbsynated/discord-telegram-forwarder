use std::sync::Arc;

use tokio::sync::RwLock;

use state::{Storage};

use crate::{config::config::Config, telegram::telegram::TelegramBot};

pub static CONFIG: Storage<RwLock<Config>> = Storage::new();

pub static TG_CLIENT: Storage<Arc<TelegramBot>> = Storage::new();
