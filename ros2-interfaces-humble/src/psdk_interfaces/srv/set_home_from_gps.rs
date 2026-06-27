use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHomeFromGPSRequest {
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for SetHomeFromGPSRequest {
    fn default() -> Self {
        SetHomeFromGPSRequest {
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl crate::Message for SetHomeFromGPSRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHomeFromGPSResponse {
    pub success: bool,
}

impl Default for SetHomeFromGPSResponse {
    fn default() -> Self {
        SetHomeFromGPSResponse { success: false }
    }
}

impl crate::Message for SetHomeFromGPSResponse {}

pub struct SetHomeFromGPS;
impl crate::Service for SetHomeFromGPS {
    type Request = SetHomeFromGPSRequest;
    type Response = SetHomeFromGPSResponse;

    fn request_type_name(&self) -> &str {
        "SetHomeFromGPSRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetHomeFromGPSResponse"
    }
}
