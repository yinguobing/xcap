use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorModeRequestRequest {
    pub radar_mode: u8,
}

impl SensorModeRequestRequest {
    pub const MODE_START_MODULATION: u8 = 1;
    pub const MODE_STOP_MODULATION: u8 = 2;
}

impl Default for SensorModeRequestRequest {
    fn default() -> Self {
        SensorModeRequestRequest { radar_mode: 0 }
    }
}

impl crate::Message for SensorModeRequestRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorModeRequestResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SensorModeRequestResponse {
    fn default() -> Self {
        SensorModeRequestResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for SensorModeRequestResponse {}

pub struct SensorModeRequest;
impl crate::Service for SensorModeRequest {
    type Request = SensorModeRequestRequest;
    type Response = SensorModeRequestResponse;

    fn request_type_name(&self) -> &str {
        "SensorModeRequestRequest"
    }
    fn response_type_name(&self) -> &str {
        "SensorModeRequestResponse"
    }
}
