use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionInfoRequest {
    pub motion_key: ::std::string::String,
}

impl Default for GetMotionInfoRequest {
    fn default() -> Self {
        GetMotionInfoRequest {
            motion_key: ::std::string::String::new(),
        }
    }
}

impl crate::Message for GetMotionInfoRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionInfoResponse {
    pub motion: crate::play_motion2_msgs::msg::Motion,
}

impl Default for GetMotionInfoResponse {
    fn default() -> Self {
        GetMotionInfoResponse {
            motion: crate::play_motion2_msgs::msg::Motion::default(),
        }
    }
}

impl crate::Message for GetMotionInfoResponse {}

pub struct GetMotionInfo;
impl crate::Service for GetMotionInfo {
    type Request = GetMotionInfoRequest;
    type Response = GetMotionInfoResponse;

    fn request_type_name(&self) -> &str {
        "GetMotionInfoRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetMotionInfoResponse"
    }
}
