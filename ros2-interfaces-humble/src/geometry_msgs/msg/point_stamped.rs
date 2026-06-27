use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PointStamped {
    pub header: crate::std_msgs::msg::Header,
    pub point: crate::geometry_msgs::msg::Point,
}

impl crate::Message for PointStamped {}
