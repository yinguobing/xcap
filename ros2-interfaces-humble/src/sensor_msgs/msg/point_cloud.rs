use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PointCloud {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::geometry_msgs::msg::Point32>,
    pub channels: Vec<crate::sensor_msgs::msg::ChannelFloat32>,
}

impl crate::Message for PointCloud {}
