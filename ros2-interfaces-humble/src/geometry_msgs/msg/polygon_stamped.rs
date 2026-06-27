use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PolygonStamped {
    pub header: crate::std_msgs::msg::Header,
    pub polygon: crate::geometry_msgs::msg::Polygon,
}

impl crate::Message for PolygonStamped {}
