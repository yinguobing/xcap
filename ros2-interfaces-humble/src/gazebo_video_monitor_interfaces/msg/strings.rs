use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Strings {
    pub names: Vec<::std::string::String>,
}

impl Default for Strings {
    fn default() -> Self {
        Strings { names: Vec::new() }
    }
}

impl crate::Message for Strings {}
