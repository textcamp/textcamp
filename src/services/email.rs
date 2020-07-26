use rusoto_core::Region;
use rusoto_sesv2::{
    Body, Content, Destination, EmailContent, Message, SendEmailRequest, SesV2, SesV2Client,
};

use log::{trace, warn};
use std::fmt;

pub struct Email {
    client: SesV2Client,
}

impl fmt::Debug for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Add in session_tokens and otp_tokens
        f.debug_struct("Email")
            .field("client", &"rusoto_sesv2::SesV2Client".to_owned())
            .finish()
    }
}

impl Default for Email {
    fn default() -> Self {
        Self::new()
    }
}

impl Email {
    pub fn new() -> Self {
        let client = SesV2Client::new(Region::default());
        Self { client }
    }

    pub async fn send(&self, to: String, cc: Option<Vec<String>>, subject: String, body: String) {
        let email_request = SendEmailRequest {
            configuration_set_name: None,
            content: EmailContent {
                raw: None,
                template: None,
                simple: Some(Message {
                    subject: Content {
                        charset: Some("utf-8".to_owned()),
                        data: subject,
                    },
                    body: Body {
                        html: None,
                        text: Some(Content {
                            charset: Some("utf-8".to_owned()),
                            data: body,
                        }),
                    },
                }),
            },
            destination: Destination {
                bcc_addresses: None,
                cc_addresses: cc,
                to_addresses: Some(vec![to.to_owned()]),
            },
            email_tags: None,
            feedback_forwarding_email_address: None,
            from_email_address: Some("play@text.camp".to_owned()),
            reply_to_addresses: None,
        };

        if !super::service_credentials() {
            warn!("Email send: no service credentials!");
            return;
        };

        match self.client.send_email(email_request).await {
            Ok(r) => trace!("{:?}", r),
            Err(e) => warn!("SEND EMAIL ERROR: {}", e),
        }
    }
}
