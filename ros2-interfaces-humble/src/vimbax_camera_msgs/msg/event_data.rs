use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    pub entries: Vec<crate::vimbax_camera_msgs::msg::EventDataEntry>,
}

impl Default for EventData {
    fn default() -> Self {
        EventData {
            entries: Vec::new(),
        }
    }
}

impl crate::Message for EventData {}
