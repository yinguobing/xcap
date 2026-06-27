use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetCameraInfoRequest {
    pub camera_info: crate::sensor_msgs::msg::CameraInfo,
}

impl crate::Message for SetCameraInfoRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SetCameraInfoResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl crate::Message for SetCameraInfoResponse {}

pub struct SetCameraInfo;
impl crate::Service for SetCameraInfo {
    type Request = SetCameraInfoRequest;
    type Response = SetCameraInfoResponse;

    fn request_type_name(&self) -> &str {
        "SetCameraInfoRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetCameraInfoResponse"
    }
}
