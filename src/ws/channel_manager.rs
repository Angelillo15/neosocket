use std::collections::HashMap;
use std::sync::Mutex;

use serde::Serialize;
use uuid::Uuid;

use crate::ws::channel::Channel;

#[derive(Debug, Serialize)]
pub struct ChannelManager {
    pub channels: Mutex<HashMap<Uuid, Channel>>,
}

impl ChannelManager {
    pub fn new() -> ChannelManager {
        ChannelManager {
            channels: Mutex::new(HashMap::new()),
        }
    }

    pub fn create_channel(&self) -> Uuid {
        let id = Uuid::new_v4();
        let channel = Channel::new(id);
        self.channels.lock().unwrap().insert(id, channel.clone());
        id
    }
}
