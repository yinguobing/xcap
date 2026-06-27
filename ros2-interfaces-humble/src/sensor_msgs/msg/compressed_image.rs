use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CompressedImage {
    pub header: crate::std_msgs::msg::Header,
    pub format: ::std::string::String,
    pub data: Vec<u8>,
}

impl crate::Message for CompressedImage {}
