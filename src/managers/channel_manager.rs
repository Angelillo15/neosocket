use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::managers::channel::Channel;

#[derive(Debug)]
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

    pub fn channel_exists(&self, id: String) -> bool {
        match Uuid::parse_str(&id) {
            Ok(uuid) => self.channels.lock().unwrap().contains_key(&uuid),
            Err(_) => false,
        }
    }

    pub fn mark_channel_as_active(&self, id: &Uuid) {
        let mut channels = self.channels.lock().unwrap();

        if let Some(channel) = channels.get_mut(&id) {
            channel.time_since_no_clients = 0;
        }
    }

    pub fn mark_channel_as_inactive(&self, id: &Uuid) {
        let mut channels = self.channels.lock().unwrap();

        if let Some(channel) = channels.get_mut(id) {
            channel.time_since_no_clients = 0;
        }
    }
}
