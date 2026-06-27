use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Int8 {
    pub data: i8,
}

impl crate::Message for Int8 {}
