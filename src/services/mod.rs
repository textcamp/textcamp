pub mod account;
pub mod db;
pub mod email;

use rusoto_core::credential::*;

/// Checks to see if there is an AWS region specified in the environment or configuration
pub fn service_credentials() -> bool {
    match std::env::var("AWS_DEFAULT_REGION").or_else(|_| std::env::var("AWS_REGION")) {
        Ok(_) => true,
        Err(_) => match ProfileProvider::region() {
            Ok(Some(_)) => true,
            _ => false,
        },
    }
}
