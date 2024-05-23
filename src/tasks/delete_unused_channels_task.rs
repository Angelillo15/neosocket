use std::env;

use actix_web::web::Data;
use chrono::Utc;
use log::{debug, info};
use tokio_schedule::{every, Job};

use crate::AppState;
use crate::utils::get_current_time_in_seconds;

pub async fn start_scheduler(app_data: Data<AppState>) {
    let task = every(10)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| async { delete_unused_channels_task(app_data.clone()).await });
    task.await;
}

pub async fn delete_unused_channels_task(app_data: Data<AppState>) {
    let channel_timeout = env::var("CHANNEL_TIMEOUT")
        .unwrap_or("30".to_string())
        .parse::<u64>()
        .unwrap_or(30);

    let mut channels_to_delete = vec![];
    let current_time = get_current_time_in_seconds();

    {
        let mut channel_manager = app_data.channel_manager.lock().unwrap();

        for (id, channel) in channel_manager
            .channels
            .lock()
            .expect("Could not lock channels")
            .iter()
        {
            if channel.time_since_no_clients == 0 {
                continue;
            }

            if channel.time_since_no_clients + channel_timeout < current_time {
                channels_to_delete.push(id.clone());
            }
        }
    }

    {
        let mut channel_manager = app_data.channel_manager.lock().unwrap();

        for id in channels_to_delete {
            channel_manager
                .channels
                .lock()
                .expect("Could not lock channels")
                .remove(&id);
            debug!("Channel {} deleted", id);
        }
    }
}
