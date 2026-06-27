use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelFloat32 {
    pub name: ::std::string::String,
    pub values: Vec<f32>,
}

impl crate::Message for ChannelFloat32 {}
