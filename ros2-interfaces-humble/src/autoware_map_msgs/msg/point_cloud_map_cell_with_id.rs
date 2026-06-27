use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudMapCellWithID {
    pub cell_id: ::std::string::String,
    pub pointcloud: crate::sensor_msgs::msg::PointCloud2,
    pub metadata: crate::autoware_map_msgs::msg::PointCloudMapCellMetaData,
}

impl Default for PointCloudMapCellWithID {
    fn default() -> Self {
        PointCloudMapCellWithID {
            cell_id: ::std::string::String::new(),
            pointcloud: crate::sensor_msgs::msg::PointCloud2::default(),
            metadata: crate::autoware_map_msgs::msg::PointCloudMapCellMetaData::default(),
        }
    }
}

impl crate::Message for PointCloudMapCellWithID {}
