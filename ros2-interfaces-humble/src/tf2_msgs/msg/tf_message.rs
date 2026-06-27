use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TFMessage {
    pub transforms: Vec<crate::geometry_msgs::msg::TransformStamped>,
}

impl Default for TFMessage {
    fn default() -> Self {
        TFMessage {
            transforms: Vec::new(),
        }
    }
}

impl crate::Message for TFMessage {}
