use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestRequest {
    pub body_name: ::std::string::String,
}

impl Default for BodyRequestRequest {
    fn default() -> Self {
        BodyRequestRequest {
            body_name: ::std::string::String::new(),
        }
    }
}

impl crate::Message for BodyRequestRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestResponse {}

impl Default for BodyRequestResponse {
    fn default() -> Self {
        BodyRequestResponse {}
    }
}

impl crate::Message for BodyRequestResponse {}

pub struct BodyRequest;
impl crate::Service for BodyRequest {
    type Request = BodyRequestRequest;
    type Response = BodyRequestResponse;

    fn request_type_name(&self) -> &str {
        "BodyRequestRequest"
    }
    fn response_type_name(&self) -> &str {
        "BodyRequestResponse"
    }
}
