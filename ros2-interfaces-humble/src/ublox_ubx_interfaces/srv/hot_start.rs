use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotStartRequest {
    pub reset_type: u8,
}

impl Default for HotStartRequest {
    fn default() -> Self {
        HotStartRequest { reset_type: 0 }
    }
}

impl crate::Message for HotStartRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotStartResponse {}

impl Default for HotStartResponse {
    fn default() -> Self {
        HotStartResponse {}
    }
}

impl crate::Message for HotStartResponse {}

pub struct HotStart;
impl crate::Service for HotStart {
    type Request = HotStartRequest;
    type Response = HotStartResponse;

    fn request_type_name(&self) -> &str {
        "HotStartRequest"
    }
    fn response_type_name(&self) -> &str {
        "HotStartResponse"
    }
}
