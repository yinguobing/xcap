use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point2Dui {
    pub x: u16,
    pub y: u16,
}

impl Default for Point2Dui {
    fn default() -> Self {
        Point2Dui { x: 0, y: 0 }
    }
}

impl crate::Message for Point2Dui {}
