use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringVec {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<::std::string::String>,
}

impl Default for StringVec {
    fn default() -> Self {
        StringVec {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}

impl crate::Message for StringVec {}
