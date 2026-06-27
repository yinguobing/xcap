use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceResponseDetailsRequest {
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}

impl Default for ServiceResponseDetailsRequest {
    fn default() -> Self {
        ServiceResponseDetailsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl crate::Message for ServiceResponseDetailsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceResponseDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for ServiceResponseDetailsResponse {
    fn default() -> Self {
        ServiceResponseDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

impl crate::Message for ServiceResponseDetailsResponse {}

pub struct ServiceResponseDetails;
impl crate::Service for ServiceResponseDetails {
    type Request = ServiceResponseDetailsRequest;
    type Response = ServiceResponseDetailsResponse;

    fn request_type_name(&self) -> &str {
        "ServiceResponseDetailsRequest"
    }
    fn response_type_name(&self) -> &str {
        "ServiceResponseDetailsResponse"
    }
}
