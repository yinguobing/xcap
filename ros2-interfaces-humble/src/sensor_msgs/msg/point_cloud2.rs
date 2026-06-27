use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PointCloud2 {
    pub header: crate::std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub fields: Vec<crate::sensor_msgs::msg::PointField>,
    pub is_bigendian: bool,
    pub point_step: u32,
    pub row_step: u32,
    pub data: Vec<u8>,
    pub is_dense: bool,
}

impl crate::Message for PointCloud2 {}
