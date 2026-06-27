use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccelStamped {
    pub header: crate::std_msgs::msg::Header,
    pub accel: crate::geometry_msgs::msg::Accel,
}

impl crate::Message for AccelStamped {}
