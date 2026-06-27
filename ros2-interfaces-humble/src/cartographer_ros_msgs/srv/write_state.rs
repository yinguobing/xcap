use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteStateRequest {
    pub filename: ::std::string::String,
    pub include_unfinished_submaps: bool,
}

impl Default for WriteStateRequest {
    fn default() -> Self {
        WriteStateRequest {
            filename: ::std::string::String::new(),
            include_unfinished_submaps: false,
        }
    }
}

impl crate::Message for WriteStateRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteStateResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
}

impl Default for WriteStateResponse {
    fn default() -> Self {
        WriteStateResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
        }
    }
}

impl crate::Message for WriteStateResponse {}

pub struct WriteState;
impl crate::Service for WriteState {
    type Request = WriteStateRequest;
    type Response = WriteStateResponse;

    fn request_type_name(&self) -> &str {
        "WriteStateRequest"
    }
    fn response_type_name(&self) -> &str {
        "WriteStateResponse"
    }
}
