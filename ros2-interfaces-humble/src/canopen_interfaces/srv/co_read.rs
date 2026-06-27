use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COReadRequest {
    pub index: u16,
    pub subindex: u8,
}

impl Default for COReadRequest {
    fn default() -> Self {
        COReadRequest {
            index: 0,
            subindex: 0,
        }
    }
}

impl crate::Message for COReadRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COReadResponse {
    pub success: bool,
    pub data: u32,
}

impl Default for COReadResponse {
    fn default() -> Self {
        COReadResponse {
            success: false,
            data: 0,
        }
    }
}

impl crate::Message for COReadResponse {}

pub struct CORead;
impl crate::Service for CORead {
    type Request = COReadRequest;
    type Response = COReadResponse;

    fn request_type_name(&self) -> &str {
        "COReadRequest"
    }
    fn response_type_name(&self) -> &str {
        "COReadResponse"
    }
}
