use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanRequest {
    pub start: crate::geometry_msgs::msg::PoseStamped,
    pub goal: crate::geometry_msgs::msg::PoseStamped,
    pub tolerance: f32,
}

impl Default for GetPlanRequest {
    fn default() -> Self {
        GetPlanRequest {
            start: crate::geometry_msgs::msg::PoseStamped::default(),
            goal: crate::geometry_msgs::msg::PoseStamped::default(),
            tolerance: 0.0,
        }
    }
}

impl crate::Message for GetPlanRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetPlanResponse {
    pub plan: crate::nav_msgs::msg::Path,
}

impl crate::Message for GetPlanResponse {}

pub struct GetPlan;
impl crate::Service for GetPlan {
    type Request = GetPlanRequest;
    type Response = GetPlanResponse;

    fn request_type_name(&self) -> &str {
        "GetPlanRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetPlanResponse"
    }
}
