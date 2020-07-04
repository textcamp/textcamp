use serde::Serialize;
use std::collections::HashMap;

#[derive(Default, Clone, Serialize, Debug)]
pub struct Markup {
    pub text: String,
    pub clicks: HashMap<String, String>,
}
