use actix::Message;
use serde::{Deserialize, Serialize};
use crate::packets::packet_container::PacketContainer;
use crate::services::ws::Handler;

#[derive(Message, Serialize, Deserialize, Clone, Debug)]
#[rtype(result = "()")]
pub struct ConnectedMessage {
    pub channel_uuid: String,
    pub connection_id: String,
}

impl actix::Handler<ConnectedMessage> for Handler {
    type Result = ();

    fn handle(&mut self, msg: ConnectedMessage, ctx: &mut Self::Context) {
        let packet_container = PacketContainer::new(0, self.client_id.to_string(), msg.clone());

        ctx.text(serde_json::to_string(&packet_container).expect("Could not serialize message"));
    }
}