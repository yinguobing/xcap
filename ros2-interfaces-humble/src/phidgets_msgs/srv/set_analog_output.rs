use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAnalogOutputRequest {
    pub index: u16,
    pub voltage: f64,
}

impl Default for SetAnalogOutputRequest {
    fn default() -> Self {
        SetAnalogOutputRequest {
            index: 0,
            voltage: 0.0,
        }
    }
}

impl crate::Message for SetAnalogOutputRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAnalogOutputResponse {
    pub success: bool,
}

impl Default for SetAnalogOutputResponse {
    fn default() -> Self {
        SetAnalogOutputResponse { success: false }
    }
}

impl crate::Message for SetAnalogOutputResponse {}

pub struct SetAnalogOutput;
impl crate::Service for SetAnalogOutput {
    type Request = SetAnalogOutputRequest;
    type Response = SetAnalogOutputResponse;

    fn request_type_name(&self) -> &str {
        "SetAnalogOutputRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetAnalogOutputResponse"
    }
}
