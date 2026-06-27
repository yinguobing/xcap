use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Num {
    pub num: i64,
}

impl Default for Num {
    fn default() -> Self {
        Num { num: 0 }
    }
}

impl crate::Message for Num {}
