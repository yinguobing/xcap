use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionIKRequest {
    pub ik_request: crate::moveit_msgs::msg::PositionIKRequest,
}

impl Default for GetPositionIKRequest {
    fn default() -> Self {
        GetPositionIKRequest {
            ik_request: crate::moveit_msgs::msg::PositionIKRequest::default(),
        }
    }
}

impl crate::Message for GetPositionIKRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionIKResponse {
    pub solution: crate::moveit_msgs::msg::RobotState,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GetPositionIKResponse {
    fn default() -> Self {
        GetPositionIKResponse {
            solution: crate::moveit_msgs::msg::RobotState::default(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl crate::Message for GetPositionIKResponse {}

pub struct GetPositionIK;
impl crate::Service for GetPositionIK {
    type Request = GetPositionIKRequest;
    type Response = GetPositionIKResponse;

    fn request_type_name(&self) -> &str {
        "GetPositionIKRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetPositionIKResponse"
    }
}
