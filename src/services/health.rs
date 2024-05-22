use actix_web::{get, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub status: &'static str,
    pub version: &'static str,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(Health {
        status: "Online",
        version: VERSION,
    })
}

pub fn endpoints(scope: Scope) -> Scope {
    return scope
        .service(index);
}