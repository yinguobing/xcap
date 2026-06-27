use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardwareInterface {
    pub name: ::std::string::String,
    pub is_available: bool,
    pub is_claimed: bool,
}

impl Default for HardwareInterface {
    fn default() -> Self {
        HardwareInterface {
            name: ::std::string::String::new(),
            is_available: false,
            is_claimed: false,
        }
    }
}

impl crate::Message for HardwareInterface {}
