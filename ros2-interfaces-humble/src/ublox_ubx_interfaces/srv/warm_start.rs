use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmStartRequest {
    pub reset_type: u8,
}

impl Default for WarmStartRequest {
    fn default() -> Self {
        WarmStartRequest { reset_type: 0 }
    }
}

impl crate::Message for WarmStartRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmStartResponse {}

impl Default for WarmStartResponse {
    fn default() -> Self {
        WarmStartResponse {}
    }
}

impl crate::Message for WarmStartResponse {}

pub struct WarmStart;
impl crate::Service for WarmStart {
    type Request = WarmStartRequest;
    type Response = WarmStartResponse;

    fn request_type_name(&self) -> &str {
        "WarmStartRequest"
    }
    fn response_type_name(&self) -> &str {
        "WarmStartResponse"
    }
}
