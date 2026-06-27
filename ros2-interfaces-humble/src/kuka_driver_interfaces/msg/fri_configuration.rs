use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriConfiguration {
    pub receive_multiplier: i32,
    pub send_period_ms: i32,
}

impl Default for FriConfiguration {
    fn default() -> Self {
        FriConfiguration {
            receive_multiplier: 0,
            send_period_ms: 0,
        }
    }
}

impl crate::Message for FriConfiguration {}
