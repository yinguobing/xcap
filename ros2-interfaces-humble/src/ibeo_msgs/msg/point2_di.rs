use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point2Di {
    pub x: i16,
    pub y: i16,
}

impl Default for Point2Di {
    fn default() -> Self {
        Point2Di { x: 0, y: 0 }
    }
}

impl crate::Message for Point2Di {}
