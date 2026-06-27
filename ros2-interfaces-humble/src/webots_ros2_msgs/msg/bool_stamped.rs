use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolStamped {
    pub header: crate::std_msgs::msg::Header,
    pub data: bool,
}

impl Default for BoolStamped {
    fn default() -> Self {
        BoolStamped {
            header: crate::std_msgs::msg::Header::default(),
            data: false,
        }
    }
}

impl crate::Message for BoolStamped {}
