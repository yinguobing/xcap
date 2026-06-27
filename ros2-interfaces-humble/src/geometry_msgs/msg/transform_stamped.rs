use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TransformStamped {
    pub header: crate::std_msgs::msg::Header,
    pub child_frame_id: ::std::string::String,
    pub transform: crate::geometry_msgs::msg::Transform,
}

impl crate::Message for TransformStamped {}
