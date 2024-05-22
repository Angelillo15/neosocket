use std::sync::{Arc, Mutex};

use crate::ws::channel_manager::ChannelManager;

pub mod services;
pub mod tasks;
pub mod ws;
pub mod utils;

pub struct AppState {
    pub channel_manager: Arc<Mutex<ChannelManager>>,
}