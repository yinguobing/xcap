use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeParameter {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for ModeParameter {
    fn default() -> Self {
        ModeParameter {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl crate::Message for ModeParameter {}
