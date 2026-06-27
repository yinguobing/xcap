use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyPlanningSceneRequest {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for ApplyPlanningSceneRequest {
    fn default() -> Self {
        ApplyPlanningSceneRequest {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

impl crate::Message for ApplyPlanningSceneRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyPlanningSceneResponse {
    pub success: bool,
}

impl Default for ApplyPlanningSceneResponse {
    fn default() -> Self {
        ApplyPlanningSceneResponse { success: false }
    }
}

impl crate::Message for ApplyPlanningSceneResponse {}

pub struct ApplyPlanningScene;
impl crate::Service for ApplyPlanningScene {
    type Request = ApplyPlanningSceneRequest;
    type Response = ApplyPlanningSceneResponse;

    fn request_type_name(&self) -> &str {
        "ApplyPlanningSceneRequest"
    }
    fn response_type_name(&self) -> &str {
        "ApplyPlanningSceneResponse"
    }
}
