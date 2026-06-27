use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Int32 {
    pub data: i32,
}

impl crate::Message for Int32 {}
