use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDigitalOutputRequest {
    pub index: u16,
    pub state: bool,
}

impl Default for SetDigitalOutputRequest {
    fn default() -> Self {
        SetDigitalOutputRequest {
            index: 0,
            state: false,
        }
    }
}

impl crate::Message for SetDigitalOutputRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDigitalOutputResponse {
    pub success: bool,
}

impl Default for SetDigitalOutputResponse {
    fn default() -> Self {
        SetDigitalOutputResponse { success: false }
    }
}

impl crate::Message for SetDigitalOutputResponse {}

pub struct SetDigitalOutput;
impl crate::Service for SetDigitalOutput {
    type Request = SetDigitalOutputRequest;
    type Response = SetDigitalOutputResponse;

    fn request_type_name(&self) -> &str {
        "SetDigitalOutputRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetDigitalOutputResponse"
    }
}
