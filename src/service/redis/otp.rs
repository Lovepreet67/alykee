use redis::{AsyncTypedCommands, SetOptions};

use crate::error::APIError;

const OTP_PREFIX: &str = "OTP";
fn get_otp_key(user_id: uuid::Uuid) -> String {
    format!("{}:{}", OTP_PREFIX, user_id)
}
pub async fn set_otp(
    client: &redis::Client,
    user_id: uuid::Uuid,
    otp: &str,
) -> Result<(), APIError> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let key = get_otp_key(user_id);
    conn.set_options(
        key,
        otp,
        SetOptions::default().with_expiration(redis::SetExpiry::EX(60)),
    )
    .await?;
    Ok(())
}

pub async fn have_active_otp(
    client: &redis::Client,
    user_id: uuid::Uuid,
) -> Result<bool, APIError> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let key = get_otp_key(user_id);
    let has: bool = conn.exists(&key).await?;
    Ok(has)
}

pub async fn get_otp(
    client: &redis::Client,
    user_id: uuid::Uuid,
) -> Result<Option<String>, APIError> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let key = get_otp_key(user_id);
    let otp: Option<String> = conn.get(key).await?;
    Ok(otp)
}

pub async fn del_otp(client: &redis::Client, user_id: uuid::Uuid) -> Result<(), APIError> {
    let mut conn = client.get_multiplexed_async_connection().await?;
    let key = get_otp_key(user_id);
    conn.del(key).await?;
    Ok(())
}
