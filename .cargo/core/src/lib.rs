use foundations::security::permissions::{PermissionToken, check_permission};
use mail_builder::MessageBuilder;
use ring::signature::{Ed25519KeyPair, KeyPair};

/// A permission token for unlocking email sending.
pub struct SendToken(Ed25519KeyPair);

impl SendToken {
    pub fn new(key: Ed25519KeyPair) -> Self { Self(key) }
    pub fn verify(&self, signature: &[u8], message: &[u8]) -> bool {
        self.0.verify(message, signature).is_ok()
    }
}

pub fn build_secure_message(to: &str, from: &str, subject: &str, body: &str, token: &SendToken) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Conceptual "unlock": Verify token before building
    if !check_permission(&token) { return Err("Unauthorized".into()); }
    Ok(MessageBuilder::new()
        .from(from)
        .to(to)
        .subject(subject)
        .text_body(body)
        .write_to_vec()?)
}
