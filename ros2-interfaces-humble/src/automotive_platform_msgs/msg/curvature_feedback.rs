use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurvatureFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub curvature: f32,
}

impl Default for CurvatureFeedback {
    fn default() -> Self {
        CurvatureFeedback {
            header: crate::std_msgs::msg::Header::default(),
            curvature: 0.0,
        }
    }
}

impl crate::Message for CurvatureFeedback {}
