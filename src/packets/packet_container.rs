use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize, Clone, Debug)]
#[rtype(result = "()")]
pub struct PacketContainer<T> {
    pub packet_id: u8,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub packet_sender: String,
    pub packet: T,
}

impl <T> PacketContainer<T> {
    pub fn new(packet_id: u8, packet_sender: String, packet: T) -> PacketContainer<T> {
        PacketContainer {
            packet_id,
            packet_sender,
            packet,
        }
    }

    pub fn new_without_sender(packet_id: u8, packet: T) -> PacketContainer<T> {
        PacketContainer {
            packet_id,
            packet_sender: String::new(),
            packet,
        }
    }
}