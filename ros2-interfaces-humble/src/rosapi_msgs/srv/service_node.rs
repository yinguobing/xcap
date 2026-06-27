use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceNodeRequest {
    pub service: ::std::string::String,
}

impl Default for ServiceNodeRequest {
    fn default() -> Self {
        ServiceNodeRequest {
            service: ::std::string::String::new(),
        }
    }
}

impl crate::Message for ServiceNodeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceNodeResponse {
    pub node: ::std::string::String,
}

impl Default for ServiceNodeResponse {
    fn default() -> Self {
        ServiceNodeResponse {
            node: ::std::string::String::new(),
        }
    }
}

impl crate::Message for ServiceNodeResponse {}

pub struct ServiceNode;
impl crate::Service for ServiceNode {
    type Request = ServiceNodeRequest;
    type Response = ServiceNodeResponse;

    fn request_type_name(&self) -> &str {
        "ServiceNodeRequest"
    }
    fn response_type_name(&self) -> &str {
        "ServiceNodeResponse"
    }
}
