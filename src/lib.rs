use std::sync::{Arc, Mutex};

use crate::managers::channel_manager::ChannelManager;
use crate::managers::recipient_manager::RecipientManager;

pub mod services;
pub mod tasks;
pub mod utils;
pub mod managers;
pub mod packets;

pub struct AppState {
    pub channel_manager: Arc<Mutex<ChannelManager>>,
    pub recipient_manager: Arc<Mutex<RecipientManager>>,
}
