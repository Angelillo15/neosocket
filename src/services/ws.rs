use actix::{Actor, Addr, Message, StreamHandler};
use actix_web_actors::ws;

struct Handler;

#[derive(Message)]
#[rtype(result = "()")]
struct Connect {
    session_id: usize,
    addr: Addr<Handler>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    session_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
struct ClientMessage {
    session_id: usize,
    message: String,
}


impl Actor for Handler {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Handler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}