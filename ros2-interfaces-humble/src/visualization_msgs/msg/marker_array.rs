use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerArray {
    pub markers: Vec<crate::visualization_msgs::msg::Marker>,
}

impl Default for MarkerArray {
    fn default() -> Self {
        MarkerArray {
            markers: Vec::new(),
        }
    }
}

impl crate::Message for MarkerArray {}
