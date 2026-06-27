use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringRequest {
    pub data: ::std::string::String,
}

impl Default for SetStringRequest {
    fn default() -> Self {
        SetStringRequest {
            data: ::std::string::String::new(),
        }
    }
}

impl crate::Message for SetStringRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetStringResponse {
    fn default() -> Self {
        SetStringResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for SetStringResponse {}

pub struct SetString;
impl crate::Service for SetString {
    type Request = SetStringRequest;
    type Response = SetStringResponse;

    fn request_type_name(&self) -> &str {
        "SetStringRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetStringResponse"
    }
}
