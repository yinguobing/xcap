use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccStateMachine {
    pub states: Vec<crate::smacc2_msgs::msg::SmaccState>,
}

impl Default for SmaccStateMachine {
    fn default() -> Self {
        SmaccStateMachine { states: Vec::new() }
    }
}

impl crate::Message for SmaccStateMachine {}
