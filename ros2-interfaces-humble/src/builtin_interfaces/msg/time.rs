use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Time {
    pub sec: i32,
    pub nanosec: u32,
}

impl crate::Message for Time {}
