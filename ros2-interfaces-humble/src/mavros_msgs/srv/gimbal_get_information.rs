use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalGetInformationRequest {}

impl Default for GimbalGetInformationRequest {
    fn default() -> Self {
        GimbalGetInformationRequest {}
    }
}

impl crate::Message for GimbalGetInformationRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalGetInformationResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalGetInformationResponse {
    fn default() -> Self {
        GimbalGetInformationResponse {
            success: false,
            result: 0,
        }
    }
}

impl crate::Message for GimbalGetInformationResponse {}

pub struct GimbalGetInformation;
impl crate::Service for GimbalGetInformation {
    type Request = GimbalGetInformationRequest;
    type Response = GimbalGetInformationResponse;

    fn request_type_name(&self) -> &str {
        "GimbalGetInformationRequest"
    }
    fn response_type_name(&self) -> &str {
        "GimbalGetInformationResponse"
    }
}
