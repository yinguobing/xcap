use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePointcloudOctomapRequest {
    pub cloud: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for UpdatePointcloudOctomapRequest {
    fn default() -> Self {
        UpdatePointcloudOctomapRequest {
            cloud: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

impl crate::Message for UpdatePointcloudOctomapRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePointcloudOctomapResponse {
    pub success: bool,
}

impl Default for UpdatePointcloudOctomapResponse {
    fn default() -> Self {
        UpdatePointcloudOctomapResponse { success: false }
    }
}

impl crate::Message for UpdatePointcloudOctomapResponse {}

pub struct UpdatePointcloudOctomap;
impl crate::Service for UpdatePointcloudOctomap {
    type Request = UpdatePointcloudOctomapRequest;
    type Response = UpdatePointcloudOctomapResponse;

    fn request_type_name(&self) -> &str {
        "UpdatePointcloudOctomapRequest"
    }
    fn response_type_name(&self) -> &str {
        "UpdatePointcloudOctomapResponse"
    }
}
