use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGoHomeAltitudeRequest {
    pub altitude: u16,
}

impl Default for SetGoHomeAltitudeRequest {
    fn default() -> Self {
        SetGoHomeAltitudeRequest { altitude: 0 }
    }
}

impl crate::Message for SetGoHomeAltitudeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGoHomeAltitudeResponse {
    pub success: bool,
}

impl Default for SetGoHomeAltitudeResponse {
    fn default() -> Self {
        SetGoHomeAltitudeResponse { success: false }
    }
}

impl crate::Message for SetGoHomeAltitudeResponse {}

pub struct SetGoHomeAltitude;
impl crate::Service for SetGoHomeAltitude {
    type Request = SetGoHomeAltitudeRequest;
    type Response = SetGoHomeAltitudeResponse;

    fn request_type_name(&self) -> &str {
        "SetGoHomeAltitudeRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetGoHomeAltitudeResponse"
    }
}
