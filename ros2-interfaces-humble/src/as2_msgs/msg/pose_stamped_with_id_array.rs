use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseStampedWithIDArray {
    pub poses: Vec<crate::as2_msgs::msg::PoseStampedWithID>,
}

impl Default for PoseStampedWithIDArray {
    fn default() -> Self {
        PoseStampedWithIDArray { poses: Vec::new() }
    }
}

impl crate::Message for PoseStampedWithIDArray {}
