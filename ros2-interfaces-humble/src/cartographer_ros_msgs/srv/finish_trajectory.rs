use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishTrajectoryRequest {
    pub trajectory_id: i32,
}

impl Default for FinishTrajectoryRequest {
    fn default() -> Self {
        FinishTrajectoryRequest { trajectory_id: 0 }
    }
}

impl crate::Message for FinishTrajectoryRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishTrajectoryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
}

impl Default for FinishTrajectoryResponse {
    fn default() -> Self {
        FinishTrajectoryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
        }
    }
}

impl crate::Message for FinishTrajectoryResponse {}

pub struct FinishTrajectory;
impl crate::Service for FinishTrajectory {
    type Request = FinishTrajectoryRequest;
    type Response = FinishTrajectoryResponse;

    fn request_type_name(&self) -> &str {
        "FinishTrajectoryRequest"
    }
    fn response_type_name(&self) -> &str {
        "FinishTrajectoryResponse"
    }
}
