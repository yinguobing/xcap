use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetPoseRequest {
    fn default() -> Self {
        SetPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl crate::Message for SetPoseRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseResponse {}

impl Default for SetPoseResponse {
    fn default() -> Self {
        SetPoseResponse {}
    }
}

impl crate::Message for SetPoseResponse {}

pub struct SetPose;
impl crate::Service for SetPose {
    type Request = SetPoseRequest;
    type Response = SetPoseResponse;

    fn request_type_name(&self) -> &str {
        "SetPoseRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetPoseResponse"
    }
}
