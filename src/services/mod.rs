pub mod accounts;
pub mod db;
pub mod email;
pub mod sessions;

use rusoto_core::credential::*;

/// Checks to see if there is an AWS region specified in the environment or configuration
pub fn service_credentials() -> bool {
    // TODO: Convert this to ChainProvider::new().credentials().is_ok() ... which is async, sigh.
    match std::env::var("AWS_DEFAULT_REGION").or_else(|_| std::env::var("AWS_REGION")) {
        Ok(_) => true,
        Err(_) => match ProfileProvider::region() {
            Ok(Some(_)) => true,
            _ => false,
        },
    }
}
