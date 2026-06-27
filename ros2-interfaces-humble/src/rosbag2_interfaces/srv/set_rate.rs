use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRateRequest {
    pub rate: f64,
}

impl Default for SetRateRequest {
    fn default() -> Self {
        SetRateRequest { rate: 0.0 }
    }
}

impl crate::Message for SetRateRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRateResponse {
    pub success: bool,
}

impl Default for SetRateResponse {
    fn default() -> Self {
        SetRateResponse { success: false }
    }
}

impl crate::Message for SetRateResponse {}

pub struct SetRate;
impl crate::Service for SetRate {
    type Request = SetRateRequest;
    type Response = SetRateResponse;

    fn request_type_name(&self) -> &str {
        "SetRateRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetRateResponse"
    }
}
