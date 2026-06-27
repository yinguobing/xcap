use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetMapRequest {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl crate::Message for SetMapRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetMapResponse {
    pub success: bool,
}

impl crate::Message for SetMapResponse {}

pub struct SetMap;
impl crate::Service for SetMap {
    type Request = SetMapRequest;
    type Response = SetMapResponse;

    fn request_type_name(&self) -> &str {
        "SetMapRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetMapResponse"
    }
}
