use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UInt32 {
    pub data: u32,
}

impl crate::Message for UInt32 {}
