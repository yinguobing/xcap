use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetFocusRingValueRequest {
    pub payload_index: u8, // default: 1
    pub value: u16,
}

impl Default for CameraSetFocusRingValueRequest {
    fn default() -> Self {
        CameraSetFocusRingValueRequest {
            payload_index: 1,
            value: 0,
        }
    }
}

impl crate::Message for CameraSetFocusRingValueRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraSetFocusRingValueResponse {
    pub success: bool,
}

impl Default for CameraSetFocusRingValueResponse {
    fn default() -> Self {
        CameraSetFocusRingValueResponse { success: false }
    }
}

impl crate::Message for CameraSetFocusRingValueResponse {}

pub struct CameraSetFocusRingValue;
impl crate::Service for CameraSetFocusRingValue {
    type Request = CameraSetFocusRingValueRequest;
    type Response = CameraSetFocusRingValueResponse;

    fn request_type_name(&self) -> &str {
        "CameraSetFocusRingValueRequest"
    }
    fn response_type_name(&self) -> &str {
        "CameraSetFocusRingValueResponse"
    }
}
