use serde::{Serialize};
use uuid::Uuid;

use crate::utils::get_current_time_in_seconds;

#[derive(Debug, Serialize, Clone)]
pub struct Channel {
    /// The unique identifier for the channel.
    /// This is a UUID.
    /// The UUID is generated when the channel is created.
    /// The UUID should be stored in the clients' session data.
    pub id: String,
    /// The time since the last client disconnected from the channel or the channel was created.
    /// This is used to determine if the channel should be deleted.
    /// If the time since the last client disconnected from the channel is greater than the
    /// `channel_timeout` value, the channel will be deleted.
    /// If the time since the channel was created is greater than the `channel_timeout` value and
    /// there are no clients connected to the channel, the channel will be deleted.
    /// The `channel_timeout` value is set in the `.env` file.
    /// The default value is 30 seconds.
    /// The `channel_timeout` value can be changed by setting the `channel_timeout` environment
    /// variable in the `.env` file.
    /// The `channel_timeout` and `time_since_no_clients` value is in seconds.
    /// If the `time_since_no_clients` value is `-1` the channel will be marked as active and will
    /// not be deleted.
    pub time_since_no_clients: u64,
    /// The list of clients connected to the channel.
    /// The list of clients is a list of UUIDs.
    /// The UUIDs are generated when the clients connect to the channel.
    /// The UUIDs should be stored in the clients' session data.
    pub clients: Vec<Uuid>,
}

impl Channel {
    pub fn new(uuid: Uuid) -> Channel {
        Channel {
            id: uuid.to_string(),
            clients: Vec::new(),
            time_since_no_clients: get_current_time_in_seconds(),
        }
    }
}
