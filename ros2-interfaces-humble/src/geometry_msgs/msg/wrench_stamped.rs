use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WrenchStamped {
    pub header: crate::std_msgs::msg::Header,
    pub wrench: crate::geometry_msgs::msg::Wrench,
}

impl crate::Message for WrenchStamped {}
