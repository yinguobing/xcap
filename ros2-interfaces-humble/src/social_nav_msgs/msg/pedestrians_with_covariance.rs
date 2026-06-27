use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PedestriansWithCovariance {
    pub header: crate::std_msgs::msg::Header,
    pub pedestrians: Vec<crate::social_nav_msgs::msg::PedestrianWithCovariance>,
}

impl Default for PedestriansWithCovariance {
    fn default() -> Self {
        PedestriansWithCovariance {
            header: crate::std_msgs::msg::Header::default(),
            pedestrians: Vec::new(),
        }
    }
}

impl crate::Message for PedestriansWithCovariance {}
