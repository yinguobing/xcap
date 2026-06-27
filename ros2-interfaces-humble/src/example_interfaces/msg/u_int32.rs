use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt32 {
    pub data: u32,
}

impl Default for UInt32 {
    fn default() -> Self {
        UInt32 { data: 0 }
    }
}

impl crate::Message for UInt32 {}
