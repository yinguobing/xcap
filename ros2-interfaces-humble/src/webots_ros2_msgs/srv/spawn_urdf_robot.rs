use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnUrdfRobotRequest {
    pub robot: crate::webots_ros2_msgs::msg::UrdfRobot,
}

impl Default for SpawnUrdfRobotRequest {
    fn default() -> Self {
        SpawnUrdfRobotRequest {
            robot: crate::webots_ros2_msgs::msg::UrdfRobot::default(),
        }
    }
}

impl crate::Message for SpawnUrdfRobotRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnUrdfRobotResponse {
    pub success: bool,
}

impl Default for SpawnUrdfRobotResponse {
    fn default() -> Self {
        SpawnUrdfRobotResponse { success: false }
    }
}

impl crate::Message for SpawnUrdfRobotResponse {}

pub struct SpawnUrdfRobot;
impl crate::Service for SpawnUrdfRobot {
    type Request = SpawnUrdfRobotRequest;
    type Response = SpawnUrdfRobotResponse;

    fn request_type_name(&self) -> &str {
        "SpawnUrdfRobotRequest"
    }
    fn response_type_name(&self) -> &str {
        "SpawnUrdfRobotResponse"
    }
}
