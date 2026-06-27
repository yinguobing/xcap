use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LaserEcho {
    pub echoes: Vec<f32>,
}

impl crate::Message for LaserEcho {}
