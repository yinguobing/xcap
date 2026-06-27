use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Int64 {
    pub data: i64,
}

impl crate::Message for Int64 {}
