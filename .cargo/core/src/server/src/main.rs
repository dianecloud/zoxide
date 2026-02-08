use core::{build_secure_message, SendToken};
use foundations::bootstrap::Application;
use lettre::{AsyncSmtpTransport, Tokio1Executor, Message, AsyncTransport};
use lettre::transport::smtp::authentication::Credentials;
use ring::rand::SystemRandom;
use ring::signature::Ed25519KeyPair;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::new("email-service")?;  // Foundations bootstrap
    app.telemetry().init()?;  // Unlock monitoring

    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)?;
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;
    let token = SendToken::new(key_pair);

    let message = build_secure_message("recipient@example.com", "sender@example.com", "Secure Greeting", "Hello, unlocked world!", &token)?;
    let creds = Credentials::new("user".to_owned(), "pass".to_owned());
    let transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.example.com")?
        .credentials(creds)
        .build();

    let email = Message::new_from_vec(message)?;
    transport.send(email).await?;

    Ok(())
}
