use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Byte {
    pub data: u8,
}

impl crate::Message for Byte {}
