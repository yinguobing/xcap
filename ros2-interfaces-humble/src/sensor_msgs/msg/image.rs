use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Image {
    pub header: crate::std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub encoding: ::std::string::String,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: Vec<u8>,
}

impl crate::Message for Image {}
