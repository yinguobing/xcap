use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapRequest {
    pub filename: crate::std_msgs::msg::String,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            filename: crate::std_msgs::msg::String::default(),
        }
    }
}

impl crate::Message for SaveMapRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapResponse {}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse {}
    }
}

impl crate::Message for SaveMapResponse {}

pub struct SaveMap;
impl crate::Service for SaveMap {
    type Request = SaveMapRequest;
    type Response = SaveMapResponse;

    fn request_type_name(&self) -> &str {
        "SaveMapRequest"
    }
    fn response_type_name(&self) -> &str {
        "SaveMapResponse"
    }
}
