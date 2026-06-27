use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VelocityStamped {
    pub header: crate::std_msgs::msg::Header,
    pub body_frame_id: ::std::string::String,
    pub reference_frame_id: ::std::string::String,
    pub velocity: crate::geometry_msgs::msg::Twist,
}

impl crate::Message for VelocityStamped {}
