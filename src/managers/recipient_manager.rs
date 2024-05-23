use std::collections::HashMap;
use actix::Recipient;
use crate::managers::PropagateMessage;

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
        for recipient in self.recipients.values() {
            recipient.do_send(PropagateMessage::new(sender.clone(), message.clone()));
        }
    }
}