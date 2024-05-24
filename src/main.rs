use std::env;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Compress, Logger, NormalizePath, TrailingSlash};
use actix_web::web::Data;
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, info};

use neosocket::{AppState, services};
use neosocket::managers::channel_manager::ChannelManager;
use neosocket::managers::recipient_manager::RecipientManager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let host: String = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port: String = env::var("PORT")
        .unwrap_or(8080.to_string())
        .parse()
        .unwrap();

    let app_state = Data::new(AppState {
        channel_manager: Arc::new(Mutex::new(ChannelManager::new())),
        recipient_manager: Arc::new(Mutex::new(RecipientManager::new())),
    });

    info!("Starting server at {}:{}", host, port);
    debug!("Running in debug mode!");

    tokio::spawn(neosocket::tasks::delete_unused_channels_task::start_scheduler(app_state.clone()));

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(Logger::default());

        app = app.app_data(app_state.clone());

        app = app.service(services::room::endpoints(web::scope("/room")));
        app = app.service(services::ws::endpoints(web::scope("/ws")));
        app = app.service(services::health::endpoints(web::scope("")));
        app
    })
    .bind(host + ":" + &*port)?
    .run()
    .await
}
