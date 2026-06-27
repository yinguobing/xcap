use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UInt64 {
    pub data: u64,
}

impl crate::Message for UInt64 {}
