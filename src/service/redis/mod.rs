mod client;
pub use client::*;
use redis::AsyncTypedCommands;

use crate::model::health_check::Status;
pub mod otp;

pub async fn test_connection(client: redis::Client) -> Status {
    match client.get_multiplexed_async_connection().await {
        Ok(mut conn) => match conn.ping().await {
            Ok(_) => Status::Ok,
            Err(e) => Status::Error(format!("{:?}", e)),
        },
        Err(e) => Status::Error(format!("{:?}", e)),
    }
}
