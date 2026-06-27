use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Vector3Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub vector: crate::geometry_msgs::msg::Vector3,
}

impl crate::Message for Vector3Stamped {}
