use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OccupancyGrid {
    pub header: crate::std_msgs::msg::Header,
    pub info: crate::nav_msgs::msg::MapMetaData,
    pub data: Vec<i8>,
}

impl crate::Message for OccupancyGrid {}
