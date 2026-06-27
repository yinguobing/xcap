use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolygonVertex {
    pub x: f32,
    pub y: f32,
}

impl Default for PolygonVertex {
    fn default() -> Self {
        PolygonVertex { x: 0.0, y: 0.0 }
    }
}

impl crate::Message for PolygonVertex {}
