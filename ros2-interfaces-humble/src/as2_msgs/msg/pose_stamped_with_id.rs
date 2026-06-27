use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseStampedWithID {
    pub id: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
}

impl Default for PoseStampedWithID {
    fn default() -> Self {
        PoseStampedWithID {
            id: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}

impl crate::Message for PoseStampedWithID {}
