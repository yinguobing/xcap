use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseRequest {
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for ResetPoseRequest {
    fn default() -> Self {
        ResetPoseRequest {
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl crate::Message for ResetPoseRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseResponse {}

impl Default for ResetPoseResponse {
    fn default() -> Self {
        ResetPoseResponse {}
    }
}

impl crate::Message for ResetPoseResponse {}

pub struct ResetPose;
impl crate::Service for ResetPose {
    type Request = ResetPoseRequest;
    type Response = ResetPoseResponse;

    fn request_type_name(&self) -> &str {
        "ResetPoseRequest"
    }
    fn response_type_name(&self) -> &str {
        "ResetPoseResponse"
    }
}
