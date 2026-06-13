use lettre::{Message, SmtpTransport, Transport};

use crate::error::APIError;

pub fn send_email(msg: Message) -> Result<&'static str, APIError> {
    let mailer = SmtpTransport::builder_dangerous("localhost")
        .port(1025)
        .build();
    mailer.send(&msg)?;
    Ok("Shared")
}
