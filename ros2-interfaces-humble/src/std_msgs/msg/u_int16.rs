use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UInt16 {
    pub data: u16,
}

impl crate::Message for UInt16 {}
