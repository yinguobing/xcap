use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRateRequest {}

impl Default for GetRateRequest {
    fn default() -> Self {
        GetRateRequest {}
    }
}

impl crate::Message for GetRateRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRateResponse {
    pub rate: f64,
}

impl Default for GetRateResponse {
    fn default() -> Self {
        GetRateResponse { rate: 0.0 }
    }
}

impl crate::Message for GetRateResponse {}

pub struct GetRate;
impl crate::Service for GetRate {
    type Request = GetRateRequest;
    type Response = GetRateResponse;

    fn request_type_name(&self) -> &str {
        "GetRateRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetRateResponse"
    }
}
