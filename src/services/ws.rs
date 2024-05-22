use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use actix::{Actor, Addr, Message, StreamHandler};
use actix_web::{get, Error, HttpRequest, HttpResponse, post, web};
use actix_web::web::Data;
use actix_web_actors::ws;
use log::debug;
use serde::Deserialize;
use uuid::Uuid;
use crate::AppState;
use crate::ws::channel_manager::ChannelManager;

#[derive(Debug, Clone)]
struct Handler {
    channel_manager: Arc<Mutex<ChannelManager>>,
    uuid: String,
    client_id: Uuid,
}

#[derive(Deserialize)]
struct Info {
    uuid: String,
}

impl Actor for Handler {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Handler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(text)) => {
                debug!("Text: {}", text);
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

#[get("/{uuid}")]
async fn index(
    info: web::Path<Info>,
    app_state: Data<AppState>,
    req: HttpRequest,
    stream: web::Payload
) -> Result<HttpResponse, Error> {
    let id = info.uuid.clone();
    let channel_manager = app_state.channel_manager.clone();

    if !channel_manager.lock().unwrap().channel_exists(id.clone()) {
        return Ok(HttpResponse::NotFound().finish());
    }

    let resp = ws::start(Handler {
        channel_manager: channel_manager.clone(),
        uuid: id,
        client_id: Uuid::new_v4(),
    }, &req, stream);

    debug!("{:?}", resp);
    resp
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope.service(index);
}
