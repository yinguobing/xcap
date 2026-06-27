use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeRequest {
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}

impl Default for ServicesForTypeRequest {
    fn default() -> Self {
        ServicesForTypeRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl crate::Message for ServicesForTypeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeResponse {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesForTypeResponse {
    fn default() -> Self {
        ServicesForTypeResponse {
            services: Vec::new(),
        }
    }
}

impl crate::Message for ServicesForTypeResponse {}

pub struct ServicesForType;
impl crate::Service for ServicesForType {
    type Request = ServicesForTypeRequest;
    type Response = ServicesForTypeResponse;

    fn request_type_name(&self) -> &str {
        "ServicesForTypeRequest"
    }
    fn response_type_name(&self) -> &str {
        "ServicesForTypeResponse"
    }
}
