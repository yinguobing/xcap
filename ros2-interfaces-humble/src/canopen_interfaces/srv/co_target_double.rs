use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COTargetDoubleRequest {
    pub target: f64,
}

impl Default for COTargetDoubleRequest {
    fn default() -> Self {
        COTargetDoubleRequest { target: 0.0 }
    }
}

impl crate::Message for COTargetDoubleRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COTargetDoubleResponse {
    pub success: bool,
}

impl Default for COTargetDoubleResponse {
    fn default() -> Self {
        COTargetDoubleResponse { success: false }
    }
}

impl crate::Message for COTargetDoubleResponse {}

pub struct COTargetDouble;
impl crate::Service for COTargetDouble {
    type Request = COTargetDoubleRequest;
    type Response = COTargetDoubleResponse;

    fn request_type_name(&self) -> &str {
        "COTargetDoubleRequest"
    }
    fn response_type_name(&self) -> &str {
        "COTargetDoubleResponse"
    }
}
