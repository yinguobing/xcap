use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct QuaternionStamped {
    pub header: crate::std_msgs::msg::Header,
    pub quaternion: crate::geometry_msgs::msg::Quaternion,
}

impl crate::Message for QuaternionStamped {}
