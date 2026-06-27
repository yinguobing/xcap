use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PoseWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
}

impl crate::Message for PoseWithCovarianceStamped {}
