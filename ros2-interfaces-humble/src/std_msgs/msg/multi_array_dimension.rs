use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MultiArrayDimension {
    pub label: ::std::string::String,
    pub size: u32,
    pub stride: u32,
}

impl crate::Message for MultiArrayDimension {}
