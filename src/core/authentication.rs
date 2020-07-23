use std::collections::{HashMap, HashSet};
use std::fmt;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use rusoto_core::Region;
use rusoto_sesv2::{
    Body, Content, Destination, EmailContent, Message, SendEmailRequest, SesV2, SesV2Client,
};

use crate::core::Identifier;

use log::{info, trace, warn};

// TODO: Associate OTP token with initiating browser
// TODO: Expire OTP tokens after 15 minutes
// TODO: Associate email address with an identifier

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
    otp_tokens: HashSet<String>,
    session_tokens: HashMap<String, Identifier>,
    ses_client: SesV2Client,
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
        let ses_client = SesV2Client::new(Region::UsWest2);
        let otp_tokens = HashSet::new();
        let session_tokens = HashMap::new();

        Self {
            ses_client,
            otp_tokens,
            session_tokens,
        }
    }

    /// Generates a random 32 character alphanumeric token
    pub fn new_token() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(32).collect()
    }

    /// Sends an OTP link to the provided e-mail address
    pub async fn start_auth(&mut self, raw_email: &str) {
        let otp_token = Self::new_token();
        self.otp_tokens.insert(otp_token.clone());
        let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL must be set");
        let email = Self::normalize_email(raw_email);
        self.send_email(email, public_url, otp_token).await;
    }

    /// Validates and deletes an OTP token
    pub fn consume_otp_token(&mut self, token: String) -> bool {
        if self.otp_tokens.contains(&token) {
            self.otp_tokens.remove(&token);
            return true;
        }
        false
    }

    /// Creates a new session token for the given identifier
    pub fn start_session(&mut self, identifier: &Identifier) -> String {
        let session_token = Self::new_token();
        self.session_tokens
            .insert(session_token.clone(), identifier.clone());
        session_token
    }

    /// If the provided token is valid, the associated Identifier is returned
    pub fn valid_session(&self, token: &str) -> Option<Identifier> {
        self.session_tokens.get(token).cloned()
    }

    /// Deletes the session
    pub fn end_session(&mut self, token: &str) {
        self.session_tokens.remove(token);
    }

    fn normalize_email(raw_email: &str) -> String {
        raw_email.trim().to_ascii_lowercase()
    }

    async fn send_email(&self, to: String, public_url: String, otp_token: String) {
        let magic_link = format!("{}/otp?token={}", public_url, otp_token);
        info!("Sending '{}' to {}", magic_link, to);

        let email_request = SendEmailRequest {
            configuration_set_name: None,
            content: EmailContent {
                raw: None,
                template: None,
                simple: Some(Message {
                    subject: Content {
                        charset: Some("utf-8".to_owned()),
                        data: "Example email!".to_owned(),
                    },
                    body: Body {
                        html: None,
                        text: Some(Content {
                            charset: Some("utf-8".to_owned()),
                            data: format!("Your magic link: {}", magic_link),
                        }),
                    },
                }),
            },
            destination: Destination {
                bcc_addresses: None,
                cc_addresses: Some(vec!["play@text.camp".to_owned()]),
                to_addresses: Some(vec![to]),
            },
            email_tags: None,
            feedback_forwarding_email_address: None,
            from_email_address: Some("play@text.camp".to_owned()),
            reply_to_addresses: None,
        };

        match self.ses_client.send_email(email_request).await {
            Ok(r) => trace!("{:?}", r),
            Err(e) => warn!("SEND EMAIL ERROR: {}", e),
        }
    }
}
