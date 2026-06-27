use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Synchronize {}

impl Default for Synchronize {
    fn default() -> Self {
        Synchronize {}
    }
}

impl crate::Message for Synchronize {}
