use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Char {
    pub data: i8,
}

impl crate::Message for Char {}
