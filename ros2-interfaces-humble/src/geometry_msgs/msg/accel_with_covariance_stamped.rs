use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccelWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub accel: crate::geometry_msgs::msg::AccelWithCovariance,
}

impl crate::Message for AccelWithCovarianceStamped {}
