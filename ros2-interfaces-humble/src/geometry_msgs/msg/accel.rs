use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Accel {
    pub linear: crate::geometry_msgs::msg::Vector3,
    pub angular: crate::geometry_msgs::msg::Vector3,
}

impl crate::Message for Accel {}
