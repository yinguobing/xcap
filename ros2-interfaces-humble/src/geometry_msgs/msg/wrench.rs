use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Wrench {
    pub force: crate::geometry_msgs::msg::Vector3,
    pub torque: crate::geometry_msgs::msg::Vector3,
}

impl crate::Message for Wrench {}
