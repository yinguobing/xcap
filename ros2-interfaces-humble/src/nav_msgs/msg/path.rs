use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Path {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::geometry_msgs::msg::PoseStamped>,
}

impl crate::Message for Path {}
