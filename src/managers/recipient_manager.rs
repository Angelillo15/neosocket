use std::collections::HashMap;

use actix::{Message, Recipient};
use serde::Serialize;

use crate::packets::propagate_packet::PropagateMessage;

pub struct RecipientManager {
    pub recipients: HashMap<String, Recipient<PropagateMessage>>,
}

impl RecipientManager {
    pub fn new() -> RecipientManager {
        RecipientManager {
            recipients: HashMap::new(),
        }
    }

    pub fn add_recipient(&mut self, id: String, recipient: Recipient<PropagateMessage>) {
        self.recipients.insert(id, recipient);
    }

    pub fn remove_recipient(&mut self, id: String) {
        self.recipients.remove(&id);
    }

    pub fn get_recipient(&self, id: String) -> Option<&Recipient<PropagateMessage>> {
        self.recipients.get(&id)
    }

    pub fn propagate_message(&self, sender: String, message: String) {
        self.propagate_message_with_id(sender, message, 1);
    }

    pub fn propagate_message_with_id(&self, sender: String, message: String, id: u8) {
        for recipient in self.recipients.values() {
            recipient.do_send(PropagateMessage {
                sender: sender.clone(),
                message: message.clone(),
                packet_id: id,
            });
        }
    }

    pub fn propagate<T>(&self, sender: String, message: &T)
    where
        T: Serialize + Message,
    {
        for recipient in self.recipients.values() {
            recipient.do_send(PropagateMessage {
                sender: sender.clone(),
                message: serde_json::to_string(&message).expect("Could not serialize message"),
                packet_id: 2,
            });
        }
    }
}
