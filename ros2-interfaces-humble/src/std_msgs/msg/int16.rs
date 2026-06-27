use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Int16 {
    pub data: i16,
}

impl crate::Message for Int16 {}
