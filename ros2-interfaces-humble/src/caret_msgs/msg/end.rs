use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct End {}

impl Default for End {
    fn default() -> Self {
        End {}
    }
}

impl crate::Message for End {}
