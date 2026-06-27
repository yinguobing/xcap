use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SonarUsage {
    pub left: bool,
    pub right: bool,
}

impl Default for SonarUsage {
    fn default() -> Self {
        SonarUsage {
            left: false,
            right: false,
        }
    }
}

impl crate::Message for SonarUsage {}
