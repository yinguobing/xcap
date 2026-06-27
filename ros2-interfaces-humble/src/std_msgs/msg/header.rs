use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Header {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub frame_id: ::std::string::String,
}

impl crate::Message for Header {}
