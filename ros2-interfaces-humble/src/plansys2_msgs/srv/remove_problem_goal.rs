use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveProblemGoalRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for RemoveProblemGoalRequest {
    fn default() -> Self {
        RemoveProblemGoalRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl crate::Message for RemoveProblemGoalRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveProblemGoalResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for RemoveProblemGoalResponse {
    fn default() -> Self {
        RemoveProblemGoalResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl crate::Message for RemoveProblemGoalResponse {}

pub struct RemoveProblemGoal;
impl crate::Service for RemoveProblemGoal {
    type Request = RemoveProblemGoalRequest;
    type Response = RemoveProblemGoalResponse;

    fn request_type_name(&self) -> &str {
        "RemoveProblemGoalRequest"
    }
    fn response_type_name(&self) -> &str {
        "RemoveProblemGoalResponse"
    }
}
