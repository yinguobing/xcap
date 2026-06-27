use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Joy {
    pub header: crate::std_msgs::msg::Header,
    pub axes: Vec<f32>,
    pub buttons: Vec<i32>,
}

impl crate::Message for Joy {}
