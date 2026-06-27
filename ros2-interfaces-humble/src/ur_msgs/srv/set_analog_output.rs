use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAnalogOutputRequest {
    pub data: crate::ur_msgs::msg::Analog,
}

impl Default for SetAnalogOutputRequest {
    fn default() -> Self {
        SetAnalogOutputRequest {
            data: crate::ur_msgs::msg::Analog::default(),
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
