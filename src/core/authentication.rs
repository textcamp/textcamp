use std::fmt;

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
    ses_client: SesV2Client,
}

impl fmt::Debug for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Authentication")
            .field("ses_client", &"rusoto_sesv2::SesV2Client".to_owned())
            .finish()
    }
}

impl Authentication {
    pub fn new() -> Self {
        let ses_client = SesV2Client::new(Region::UsWest2);
        Self { ses_client }
    }

    pub fn start_auth(&self, raw_email: &str) {
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
                            data: "This is an example email from start_auth!".to_owned(),
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

        match tokio::Runtime::block_on(self.ses_client.send_email(email_request)) {
            Ok(resp) => trace!("{:?}", resp),
            Err(e) => trace!("{:?}", e),
        }
    }

    pub fn finish_auth(&self, token: String) {}

    fn normalize_email(raw_email: &str) -> String {
        raw_email.trim().to_ascii_lowercase()
    }
}
