use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Transform {
    pub translation: crate::geometry_msgs::msg::Vector3,
    pub rotation: crate::geometry_msgs::msg::Quaternion,
}

impl crate::Message for Transform {}
