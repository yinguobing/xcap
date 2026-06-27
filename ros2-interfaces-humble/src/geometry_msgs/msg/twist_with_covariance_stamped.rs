use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TwistWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub twist: crate::geometry_msgs::msg::TwistWithCovariance,
}

impl crate::Message for TwistWithCovarianceStamped {}
