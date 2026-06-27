use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanningSceneRequest {
    pub components: crate::moveit_msgs::msg::PlanningSceneComponents,
}

impl Default for GetPlanningSceneRequest {
    fn default() -> Self {
        GetPlanningSceneRequest {
            components: crate::moveit_msgs::msg::PlanningSceneComponents::default(),
        }
    }
}

impl crate::Message for GetPlanningSceneRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanningSceneResponse {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for GetPlanningSceneResponse {
    fn default() -> Self {
        GetPlanningSceneResponse {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

impl crate::Message for GetPlanningSceneResponse {}

pub struct GetPlanningScene;
impl crate::Service for GetPlanningScene {
    type Request = GetPlanningSceneRequest;
    type Response = GetPlanningSceneResponse;

    fn request_type_name(&self) -> &str {
        "GetPlanningSceneRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetPlanningSceneResponse"
    }
}
