use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dummy {}

impl Default for Dummy {
    fn default() -> Self {
        Dummy {}
    }
}

impl crate::Message for Dummy {}
