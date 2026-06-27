use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotArray {
    pub robots: Vec<crate::soccer_object_msgs::msg::Robot>,
}

impl Default for RobotArray {
    fn default() -> Self {
        RobotArray { robots: Vec::new() }
    }
}

impl crate::Message for RobotArray {}
