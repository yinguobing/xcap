use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientRecordingSetCurrentPoseRequest {
    pub pose: crate::geometry_msgs::msg::Pose2D,
}

impl Default for ClientRecordingSetCurrentPoseRequest {
    fn default() -> Self {
        ClientRecordingSetCurrentPoseRequest {
            pose: crate::geometry_msgs::msg::Pose2D::default(),
        }
    }
}

impl crate::Message for ClientRecordingSetCurrentPoseRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientRecordingSetCurrentPoseResponse {}

impl Default for ClientRecordingSetCurrentPoseResponse {
    fn default() -> Self {
        ClientRecordingSetCurrentPoseResponse {}
    }
}

impl crate::Message for ClientRecordingSetCurrentPoseResponse {}

pub struct ClientRecordingSetCurrentPose;
impl crate::Service for ClientRecordingSetCurrentPose {
    type Request = ClientRecordingSetCurrentPoseRequest;
    type Response = ClientRecordingSetCurrentPoseResponse;

    fn request_type_name(&self) -> &str {
        "ClientRecordingSetCurrentPoseRequest"
    }
    fn response_type_name(&self) -> &str {
        "ClientRecordingSetCurrentPoseResponse"
    }
}
