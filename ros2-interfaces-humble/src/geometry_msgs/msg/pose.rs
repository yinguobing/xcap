use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Pose {
    pub position: crate::geometry_msgs::msg::Point,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
}

impl crate::Message for Pose {}
