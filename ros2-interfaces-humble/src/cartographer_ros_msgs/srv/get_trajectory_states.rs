use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrajectoryStatesRequest {}

impl Default for GetTrajectoryStatesRequest {
    fn default() -> Self {
        GetTrajectoryStatesRequest {}
    }
}

impl crate::Message for GetTrajectoryStatesRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrajectoryStatesResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory_states: crate::cartographer_ros_msgs::msg::TrajectoryStates,
}

impl Default for GetTrajectoryStatesResponse {
    fn default() -> Self {
        GetTrajectoryStatesResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory_states: crate::cartographer_ros_msgs::msg::TrajectoryStates::default(),
        }
    }
}

impl crate::Message for GetTrajectoryStatesResponse {}

pub struct GetTrajectoryStates;
impl crate::Service for GetTrajectoryStates {
    type Request = GetTrajectoryStatesRequest;
    type Response = GetTrajectoryStatesResponse;

    fn request_type_name(&self) -> &str {
        "GetTrajectoryStatesRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetTrajectoryStatesResponse"
    }
}
