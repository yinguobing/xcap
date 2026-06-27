use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Polygon {
    pub points: Vec<crate::geometry_msgs::msg::Point32>,
}

impl crate::Message for Polygon {}
