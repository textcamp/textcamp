use std::collections::HashMap;
use std::fmt;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::core::Identifier;
use crate::services::email::Email;
use crate::services::sessions::Session;

use log::{info, warn};

// TODO: Associate OTP token with initiating browser
// TODO: Expire OTP tokens after 15 minutes

/// Authentication is done by an e-mailed "magic link"
///
/// The player provides their e-mail address, which is passed to
/// the `start_auth` method, which begins the process.
///
/// The player then receives an e-mail at the specified address,
/// containing a link. The link contains a one-time-use token,
/// which is passed to the `consume_otp_token` method when the player
/// clicks through.
///
/// After that, a session is created with `start_session` and validated
/// on subsequent connections with `valid_session`.
///
/// When a player signs out, `end_session` removes their session token.
pub struct Authentication {
    otp_tokens: HashMap<String, String>, // token -> email
    email_client: Email,
}

impl fmt::Debug for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Add in session_tokens and otp_tokens
        f.debug_struct("Authentication")
            .field("ses_client", &"rusoto_sesv2::SesV2Client".to_owned())
            .finish()
    }
}

impl Default for Authentication {
    fn default() -> Self {
        Self::new()
    }
}

impl Authentication {
    /// Returns a new Authentication instance
    pub fn new() -> Self {
        let email_client = Email::new();
        let otp_tokens = HashMap::new();

        Self {
            email_client,
            otp_tokens,
        }
    }

    /// Generates a random 32 character alphanumeric token
    pub fn new_token() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(32).collect()
    }

    /// Sends an OTP link to the provided e-mail address
    pub async fn start_auth(&mut self, raw_email: &str) {
        let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL must be set");
        let otp_token = Self::new_token();
        let email = Self::normalize_email(raw_email);
        self.send_email(&email, public_url, &otp_token).await;
        self.otp_tokens.insert(otp_token, email);
    }

    /// Validates and deletes an OTP token
    pub fn consume_otp_token(&mut self, token: String) -> Option<String> {
        let email = self.otp_tokens.remove(&token)?;
        Some(email)
    }

    /// Creates a new session token for the given identifier
    pub async fn start_session(&mut self, identifier: &Identifier) -> String {
        let session = Session {
            token: Self::new_token(),
            identifier: identifier.clone(),
        };

        let db = crate::services::db::Dynamo::new();
        if let Err(e) = db.sessions.put::<Session>(&session).await {
            warn!("START_SESSION: {:?}", e);
        }

        session.token.clone()
    }

    /// If the provided token is valid, the associated Identifier is returned
    pub async fn valid_session(&self, token: &str) -> Option<Identifier> {
        let db = crate::services::db::Dynamo::new();
        match db.sessions.get::<Session>(token).await {
            Some(s) => Some(s.identifier),
            None => None,
        }
    }

    /// Deletes the session
    pub async fn end_session(&mut self, token: &str) {
        let db = crate::services::db::Dynamo::new();
        db.sessions.delete(token).await;
    }

    fn normalize_email(raw_email: &str) -> String {
        raw_email.trim().to_ascii_lowercase()
    }

    async fn send_email(&self, to: &str, public_url: String, otp_token: &str) {
        let magic_link = format!("{}/otp?token={}", public_url, otp_token);
        info!("Sending '{}' to {}", magic_link, to);

        if std::env::var("NO_EMAIL").is_ok() {
            info!("NO_EMAIL is set; not sending e-mail.");
            return;
        }

        let cc = Some(vec!["play@text.camp".to_owned()]);
        let subject = "🏕 Welcome to Textcamp!".to_owned();
        let body = format!(
            "Here's your magic link: {}\n\nThis link only works once, so savor the moment!",
            magic_link
        );

        self.email_client
            .send(to.to_owned(), cc, subject, body)
            .await;
    }
}
