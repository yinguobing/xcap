use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Default for LedColor {
    fn default() -> Self {
        LedColor {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl crate::Message for LedColor {}
