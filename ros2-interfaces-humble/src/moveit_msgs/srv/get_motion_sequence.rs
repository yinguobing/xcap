use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionSequenceRequest {
    pub request: crate::moveit_msgs::msg::MotionSequenceRequest,
}

impl Default for GetMotionSequenceRequest {
    fn default() -> Self {
        GetMotionSequenceRequest {
            request: crate::moveit_msgs::msg::MotionSequenceRequest::default(),
        }
    }
}

impl crate::Message for GetMotionSequenceRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionSequenceResponse {
    pub response: crate::moveit_msgs::msg::MotionSequenceResponse,
}

impl Default for GetMotionSequenceResponse {
    fn default() -> Self {
        GetMotionSequenceResponse {
            response: crate::moveit_msgs::msg::MotionSequenceResponse::default(),
        }
    }
}

impl crate::Message for GetMotionSequenceResponse {}

pub struct GetMotionSequence;
impl crate::Service for GetMotionSequence {
    type Request = GetMotionSequenceRequest;
    type Response = GetMotionSequenceResponse;

    fn request_type_name(&self) -> &str {
        "GetMotionSequenceRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetMotionSequenceResponse"
    }
}
