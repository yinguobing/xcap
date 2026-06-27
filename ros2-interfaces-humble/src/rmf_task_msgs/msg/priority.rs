use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Priority {
    pub value: u64, // default: 0
}

impl Default for Priority {
    fn default() -> Self {
        Priority { value: 0 }
    }
}

impl crate::Message for Priority {}
