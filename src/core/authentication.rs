use std::collections::HashSet;
use std::fmt;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use rusoto_core::Region;
use rusoto_sesv2::{
    Body, Content, Destination, EmailContent, Message, SendEmailRequest, SesV2, SesV2Client,
};

use log::trace;

/// Authentication is done by an e-mailed "magic link"
///
/// The player provides their e-mail address, which is passed to
/// the `start_auth` method, which begins the process.
///
/// The player then receives an e-mail at the specified address,
/// containing a link. The link contains a one-time-use token,
/// which is passed to the `finish_auth` method when the player
/// clicks through.
///
/// If the token is good, then a `session token` is returned and
/// sent to the client for long term storage.
pub struct Authentication {
    session_tokens: HashSet<String>,
    ses_client: SesV2Client,
}

impl fmt::Debug for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub fn new() -> Self {
        let ses_client = SesV2Client::new(Region::UsWest2);
        let session_tokens = HashSet::new();

        Self {
            ses_client,
            session_tokens,
        }
    }

    pub fn new_token() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(32).collect()
    }

    pub async fn start_auth(&mut self, raw_email: &str) {
        let token = Self::new_token();
        self.session_tokens.insert(token.clone());

        let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL must be set");

        let email = Self::normalize_email(raw_email);

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
                            data: format!(
                                "Your magic link: {}/?sessionToken={}",
                                public_url, token
                            ),
                        }),
                    },
                }),
            },
            destination: Destination {
                bcc_addresses: Some(vec!["play@text.camp".to_owned()]),
                cc_addresses: None,
                to_addresses: Some(vec![email]),
            },
            email_tags: None,
            feedback_forwarding_email_address: None,
            from_email_address: Some("play@text.camp".to_owned()),
            reply_to_addresses: None,
        };

        let result = self.ses_client.send_email(email_request).await;
        trace!("{:?}", result);
    }

    pub fn finish_auth(&mut self, token: String) -> bool {
        if self.session_tokens.contains(&token) {
            self.session_tokens.remove(&token);
            return true;
        }
        false
    }

    fn normalize_email(raw_email: &str) -> String {
        raw_email.trim().to_ascii_lowercase()
    }
}
