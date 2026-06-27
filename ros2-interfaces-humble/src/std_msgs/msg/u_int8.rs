use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UInt8 {
    pub data: u8,
}

impl crate::Message for UInt8 {}
