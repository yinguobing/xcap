use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModeRequest {}

impl Default for GetModeRequest {
    fn default() -> Self {
        GetModeRequest {}
    }
}

impl crate::Message for GetModeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModeResponse {
    pub current_mode: ::std::string::String,
}

impl Default for GetModeResponse {
    fn default() -> Self {
        GetModeResponse {
            current_mode: ::std::string::String::new(),
        }
    }
}

impl crate::Message for GetModeResponse {}

pub struct GetMode;
impl crate::Service for GetMode {
    type Request = GetModeRequest;
    type Response = GetModeResponse;

    fn request_type_name(&self) -> &str {
        "GetModeRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetModeResponse"
    }
}
