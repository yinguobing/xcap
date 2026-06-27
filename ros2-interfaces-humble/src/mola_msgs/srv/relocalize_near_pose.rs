use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeNearPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for RelocalizeNearPoseRequest {
    fn default() -> Self {
        RelocalizeNearPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl crate::Message for RelocalizeNearPoseRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeNearPoseResponse {
    pub accepted: bool,
}

impl Default for RelocalizeNearPoseResponse {
    fn default() -> Self {
        RelocalizeNearPoseResponse { accepted: false }
    }
}

impl crate::Message for RelocalizeNearPoseResponse {}

pub struct RelocalizeNearPose;
impl crate::Service for RelocalizeNearPose {
    type Request = RelocalizeNearPoseRequest;
    type Response = RelocalizeNearPoseResponse;

    fn request_type_name(&self) -> &str {
        "RelocalizeNearPoseRequest"
    }
    fn response_type_name(&self) -> &str {
        "RelocalizeNearPoseResponse"
    }
}
