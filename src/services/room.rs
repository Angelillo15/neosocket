use actix_web::{get, HttpResponse, post, Scope};
use actix_web::web::Data;

use crate::AppState;

#[get("/")]
#[cfg(debug_assertions)]
pub async fn index(app_data: Data<AppState>) -> HttpResponse {
    use crate::managers::channel::Channel;

    let channel_manager = app_data.channel_manager.clone();
    let channels = channel_manager.lock().unwrap();
    let channels: Vec<Channel> = channels
        .channels
        .lock()
        .unwrap()
        .values()
        .cloned()
        .collect();

    HttpResponse::Ok().json(channels.clone())
}

#[get("/")]
#[cfg(not(debug_assertions))]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/")]
pub async fn create(app_data: Data<AppState>) -> HttpResponse {
    let channel_manager = app_data.channel_manager.lock().unwrap();
    let id = channel_manager.create_channel();

    HttpResponse::Ok().json(id)
}

pub fn endpoints(scope: Scope) -> Scope {
    scope.service(index).service(create)
}
