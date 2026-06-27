use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct JoyFeedbackArray {
    pub array: Vec<crate::sensor_msgs::msg::JoyFeedback>,
}

impl crate::Message for JoyFeedbackArray {}
