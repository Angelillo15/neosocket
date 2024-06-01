use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize, Clone, Debug)]
#[rtype(result = "()")]
pub struct PacketContainer<T> {
    pub packet_id: u8,
    pub packet: T,
}
