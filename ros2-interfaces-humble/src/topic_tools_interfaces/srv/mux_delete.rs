use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxDeleteRequest {
    pub topic: ::std::string::String,
}

impl Default for MuxDeleteRequest {
    fn default() -> Self {
        MuxDeleteRequest {
            topic: ::std::string::String::new(),
        }
    }
}

impl crate::Message for MuxDeleteRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxDeleteResponse {
    pub success: bool,
}

impl Default for MuxDeleteResponse {
    fn default() -> Self {
        MuxDeleteResponse { success: false }
    }
}

impl crate::Message for MuxDeleteResponse {}

pub struct MuxDelete;
impl crate::Service for MuxDelete {
    type Request = MuxDeleteRequest;
    type Response = MuxDeleteResponse;

    fn request_type_name(&self) -> &str {
        "MuxDeleteRequest"
    }
    fn response_type_name(&self) -> &str {
        "MuxDeleteResponse"
    }
}
