use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainTypesRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainTypesRequest {
    fn default() -> Self {
        GetDomainTypesRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl crate::Message for GetDomainTypesRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainTypesResponse {
    pub success: bool,
    pub types: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainTypesResponse {
    fn default() -> Self {
        GetDomainTypesResponse {
            success: false,
            types: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl crate::Message for GetDomainTypesResponse {}

pub struct GetDomainTypes;
impl crate::Service for GetDomainTypes {
    type Request = GetDomainTypesRequest;
    type Response = GetDomainTypesResponse;

    fn request_type_name(&self) -> &str {
        "GetDomainTypesRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetDomainTypesResponse"
    }
}
