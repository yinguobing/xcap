use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObstacleAvoidanceRequest {}

impl Default for GetObstacleAvoidanceRequest {
    fn default() -> Self {
        GetObstacleAvoidanceRequest {}
    }
}

impl crate::Message for GetObstacleAvoidanceRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObstacleAvoidanceResponse {
    pub obstacle_avoidance_on: bool,
    pub success: bool,
}

impl Default for GetObstacleAvoidanceResponse {
    fn default() -> Self {
        GetObstacleAvoidanceResponse {
            obstacle_avoidance_on: false,
            success: false,
        }
    }
}

impl crate::Message for GetObstacleAvoidanceResponse {}

pub struct GetObstacleAvoidance;
impl crate::Service for GetObstacleAvoidance {
    type Request = GetObstacleAvoidanceRequest;
    type Response = GetObstacleAvoidanceResponse;

    fn request_type_name(&self) -> &str {
        "GetObstacleAvoidanceRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetObstacleAvoidanceResponse"
    }
}
