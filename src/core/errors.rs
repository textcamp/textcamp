#[derive(Debug)]
pub enum TCError {
    Fatal(String),
    User(String),
    System(String),
}

impl TCError {
    pub fn fatal(message: &str) -> Self {
        TCError::Fatal(message.to_owned())
    }

    pub fn user(message: &str) -> Self {
        TCError::User(message.to_owned())
    }

    pub fn system(message: &str) -> Self {
        TCError::System(message.to_owned())
    }
}
