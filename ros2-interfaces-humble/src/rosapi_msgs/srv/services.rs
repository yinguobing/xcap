use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesRequest {}

impl Default for ServicesRequest {
    fn default() -> Self {
        ServicesRequest {}
    }
}

impl crate::Message for ServicesRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesResponse {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesResponse {
    fn default() -> Self {
        ServicesResponse {
            services: Vec::new(),
        }
    }
}

impl crate::Message for ServicesResponse {}

pub struct Services;
impl crate::Service for Services {
    type Request = ServicesRequest;
    type Response = ServicesResponse;

    fn request_type_name(&self) -> &str {
        "ServicesRequest"
    }
    fn response_type_name(&self) -> &str {
        "ServicesResponse"
    }
}
