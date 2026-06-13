use lettre::{Message, message::header::ContentType};
use rand::RngExt;

use crate::error::APIError;

pub fn get_otp(len: Option<u32>) -> String {
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::rng();
    (0..len.unwrap_or(6))
        .map(|_| {
            let idx: u8 = rng.random();
            CHARSET[idx as usize % CHARSET.len()] as char
        })
        .collect()
}

pub fn generate_otp_message(
    target_user_name: &str,
    target_user_email: &str,
    otp: &str,
) -> Result<Message, APIError> {
    let sender_address = "<test@gmail.com>".parse()?;
    let reciever_address = format!("{} <{}>", target_user_name, target_user_email).parse()?;
    let message = Message::builder()
        .subject("Password Reset")
        .from(sender_address)
        .to(reciever_address)
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Please use the {} OTP for password reset", otp))?;
    Ok(message)
}

#[cfg(test)]
mod test {
    use crate::utilities::otp::get_otp;

    #[test]
    fn test_otp() {
        println!("{}", get_otp(None));
        println!("{}", get_otp(Some(10)));
    }
}
