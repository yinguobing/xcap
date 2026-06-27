use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolygonList {
    pub polygons: Vec<crate::geometry_msgs::msg::PolygonStamped>,
}

impl Default for PolygonList {
    fn default() -> Self {
        PolygonList {
            polygons: Vec::new(),
        }
    }
}

impl crate::Message for PolygonList {}
