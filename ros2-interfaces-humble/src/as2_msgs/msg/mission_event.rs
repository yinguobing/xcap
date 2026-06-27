use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionEvent {
    pub header: crate::std_msgs::msg::Header,
    pub data: ::std::string::String,
}

impl Default for MissionEvent {
    fn default() -> Self {
        MissionEvent {
            header: crate::std_msgs::msg::Header::default(),
            data: ::std::string::String::new(),
        }
    }
}

impl crate::Message for MissionEvent {}
