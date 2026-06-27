use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListMotionsRequest {}

impl Default for ListMotionsRequest {
    fn default() -> Self {
        ListMotionsRequest {}
    }
}

impl crate::Message for ListMotionsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListMotionsResponse {
    pub motion_keys: Vec<::std::string::String>,
}

impl Default for ListMotionsResponse {
    fn default() -> Self {
        ListMotionsResponse {
            motion_keys: Vec::new(),
        }
    }
}

impl crate::Message for ListMotionsResponse {}

pub struct ListMotions;
impl crate::Service for ListMotions {
    type Request = ListMotionsRequest;
    type Response = ListMotionsResponse;

    fn request_type_name(&self) -> &str {
        "ListMotionsRequest"
    }
    fn response_type_name(&self) -> &str {
        "ListMotionsResponse"
    }
}
