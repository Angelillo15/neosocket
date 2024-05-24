


use actix::{Actor, AsyncContext, Recipient, StreamHandler};
use actix_web::web::Data;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{debug, error};
use serde::{Deserialize};
use uuid::Uuid;


use crate::managers::{ConnectedMessage, PropagateMessage};
use crate::{AppState};

#[derive(Clone)]
pub struct Handler {
    pub app_state: Data<AppState>,
    uuid: String,
    client_id: Uuid,
}

#[derive(Deserialize)]
struct Info {
    uuid: String,
}

impl Actor for Handler {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address().clone();
        let recipient = addr.clone().recipient();

        {
            let connected_recipient: Recipient<ConnectedMessage> = addr.clone().recipient();

            connected_recipient.do_send(ConnectedMessage {
                channel_uuid: self.uuid.clone().parse().expect("Could not parse uuid"),
                connection_id: self.client_id.to_string(),
            });
        }

        let app_state = self.app_state.clone();

        {
            let mut recipient_manager = app_state.recipient_manager.lock().unwrap();
            recipient_manager.add_recipient(self.client_id.to_string(), recipient);
            debug!("Added recipient: {:?}", self.client_id);
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Handler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(text)) => {
                let app_state = self.app_state.clone();
                let recipient_manager = app_state.recipient_manager.clone();

                {
                    let recipient_manager = recipient_manager.lock().unwrap();
                    recipient_manager.propagate_message(
                        self.client_id.to_string(),
                        text.clone().parse().expect("Could not parse message"),
                    );
                }

                debug!("Text: {}", text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                let app_state = self.app_state.clone();

                {
                    let mut recipient_manager = app_state.recipient_manager.lock().unwrap();
                    recipient_manager.remove_recipient(self.client_id.to_string());

                    if recipient_manager.recipients.len() == 0 {
                        let channel_manager = app_state.channel_manager.clone();
                        let channel_manager = channel_manager.lock().unwrap();
                        let uuid = Uuid::parse_str(&self.uuid).unwrap();
                        channel_manager.mark_channel_as_inactive(uuid);
                    }

                    debug!("Removed recipient: {:?}", self.client_id);
                }

                debug!("Closing connection: {:?}", self.client_id);
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

impl actix::Handler<PropagateMessage> for Handler {
    type Result = ();

    fn handle(&mut self, msg: PropagateMessage, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).expect("Could not serialize message"));
    }
}

impl actix::Handler<ConnectedMessage> for Handler {
    type Result = ();

    fn handle(&mut self, msg: ConnectedMessage, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).expect("Could not serialize message"));
    }
}

#[get("/{uuid}")]
async fn index(
    info: web::Path<Info>,
    app_state: Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let id = info.uuid.clone();
    let channel_manager = app_state.channel_manager.clone();

    let resp = ws::start(
        Handler {
            app_state: app_state.clone(),
            uuid: id.clone(),
            client_id: Uuid::new_v4(),
        },
        &req,
        stream,
    );

    return match resp {
        Ok(_) => {
            let channel_manager = channel_manager.lock().unwrap();

            if !channel_manager.channel_exists(id.clone()) {
                return Ok(HttpResponse::NotFound().finish());
            }

            match Uuid::parse_str(&id) {
                Ok(uuid) => {
                    channel_manager.mark_channel_as_active(uuid);
                }
                Err(_) => {
                    error!("Could not parse uuid: {}", id.clone());
                    return Ok(HttpResponse::NotFound().finish());
                }
            }

            debug!("Connection established: {:?}", id);
            debug!("Response: {:?}", resp);
            resp
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(HttpResponse::from_error(e))
        }
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope.service(index);
}
