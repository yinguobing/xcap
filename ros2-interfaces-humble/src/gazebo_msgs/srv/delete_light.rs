use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLightRequest {
    pub light_name: ::std::string::String,
}

impl Default for DeleteLightRequest {
    fn default() -> Self {
        DeleteLightRequest {
            light_name: ::std::string::String::new(),
        }
    }
}

impl crate::Message for DeleteLightRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLightResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteLightResponse {
    fn default() -> Self {
        DeleteLightResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for DeleteLightResponse {}

pub struct DeleteLight;
impl crate::Service for DeleteLight {
    type Request = DeleteLightRequest;
    type Response = DeleteLightResponse;

    fn request_type_name(&self) -> &str {
        "DeleteLightRequest"
    }
    fn response_type_name(&self) -> &str {
        "DeleteLightResponse"
    }
}
