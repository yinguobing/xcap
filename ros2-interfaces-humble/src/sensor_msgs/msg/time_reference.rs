use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeReference {
    pub header: crate::std_msgs::msg::Header,
    pub time_ref: crate::builtin_interfaces::msg::Time,
    pub source: ::std::string::String,
}

impl crate::Message for TimeReference {}
