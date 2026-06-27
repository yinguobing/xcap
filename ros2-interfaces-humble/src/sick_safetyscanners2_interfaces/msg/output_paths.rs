use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputPaths {
    pub status: Vec<bool>,
    pub is_safe: Vec<bool>,
    pub is_valid: Vec<bool>,
    pub active_monitoring_case: i32,
}

impl Default for OutputPaths {
    fn default() -> Self {
        OutputPaths {
            status: Vec::new(),
            is_safe: Vec::new(),
            is_valid: Vec::new(),
            active_monitoring_case: 0,
        }
    }
}

impl crate::Message for OutputPaths {}
