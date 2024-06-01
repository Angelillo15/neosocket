use actix::Message;
use serde::{Deserialize, Serialize};

use crate::packets::packet_container::PacketContainer;
use crate::services::ws::Handler;

#[derive(Message, Serialize, Deserialize, Clone, Debug)]
#[rtype(result = "()")]
pub struct PropagateMessage {
    pub sender: String,
    pub message: String,
    #[serde(skip_serializing)]
    pub packet_id: u8,
}

impl PropagateMessage {
    pub fn new(sender: String, message: String) -> PropagateMessage {
        PropagateMessage {
            sender,
            message,
            packet_id: 1,
        }
    }

    pub fn new_with_id(sender: String, message: String, packet_id: u8) -> PropagateMessage {
        PropagateMessage {
            sender,
            message,
            packet_id,
        }
    }
}

impl actix::Handler<PropagateMessage> for Handler {
    type Result = ();

    fn handle(&mut self, msg: PropagateMessage, ctx: &mut Self::Context) {
        let pm = msg.clone();

        let packet_container = PacketContainer {
            packet: msg.message,
            packet_id: pm.packet_id,
            packet_sender: pm.sender,
        };

        ctx.text(serde_json::to_string(&packet_container).expect("Could not serialize message"));
    }
}
