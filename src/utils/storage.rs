use std::{sync::{RwLock}};

use state::Storage;

use crate::config::config::Config;

pub static CONFIG: Storage<RwLock<Config>> = Storage::new();
