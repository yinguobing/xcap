use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryConstraints {
    pub constraints: Vec<crate::moveit_msgs::msg::Constraints>,
}

impl Default for TrajectoryConstraints {
    fn default() -> Self {
        TrajectoryConstraints {
            constraints: Vec::new(),
        }
    }
}

impl crate::Message for TrajectoryConstraints {}
