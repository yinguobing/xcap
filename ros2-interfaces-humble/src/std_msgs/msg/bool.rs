use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Bool {
    pub data: bool,
}

impl crate::Message for Bool {}
