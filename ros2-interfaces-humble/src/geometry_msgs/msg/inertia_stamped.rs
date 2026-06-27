use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InertiaStamped {
    pub header: crate::std_msgs::msg::Header,
    pub inertia: crate::geometry_msgs::msg::Inertia,
}

impl crate::Message for InertiaStamped {}
