use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MultiArrayLayout {
    pub dim: Vec<crate::std_msgs::msg::MultiArrayDimension>,
    pub data_offset: u32,
}

impl crate::Message for MultiArrayLayout {}
