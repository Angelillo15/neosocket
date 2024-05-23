use actix::{Message};
use serde::{Deserialize, Serialize};

pub mod channel;
pub mod channel_manager;
pub mod recipient_manager;

#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct PropagateMessage {
    pub sender: String,
    pub message: String,
}

#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct ConnectedMessage {
    pub channel_uuid: String,
    pub connection_id: String,
}

impl PropagateMessage {
    pub fn new(sender: String, message: String) -> PropagateMessage {
        PropagateMessage {
            sender,
            message,
        }
    }
}