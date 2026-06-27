use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteeringFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub steering_wheel_angle: f32,
}

impl Default for SteeringFeedback {
    fn default() -> Self {
        SteeringFeedback {
            header: crate::std_msgs::msg::Header::default(),
            steering_wheel_angle: 0.0,
        }
    }
}

impl crate::Message for SteeringFeedback {}
