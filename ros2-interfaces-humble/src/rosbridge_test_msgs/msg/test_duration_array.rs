use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestDurationArray {
    pub durations: Vec<crate::builtin_interfaces::msg::Duration>,
}

impl Default for TestDurationArray {
    fn default() -> Self {
        TestDurationArray {
            durations: Vec::new(),
        }
    }
}

impl crate::Message for TestDurationArray {}
