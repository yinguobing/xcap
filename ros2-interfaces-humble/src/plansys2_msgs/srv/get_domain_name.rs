use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainNameRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainNameRequest {
    fn default() -> Self {
        GetDomainNameRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl crate::Message for GetDomainNameRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainNameResponse {
    pub success: bool,
    pub name: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainNameResponse {
    fn default() -> Self {
        GetDomainNameResponse {
            success: false,
            name: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl crate::Message for GetDomainNameResponse {}

pub struct GetDomainName;
impl crate::Service for GetDomainName {
    type Request = GetDomainNameRequest;
    type Response = GetDomainNameResponse;

    fn request_type_name(&self) -> &str {
        "GetDomainNameRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetDomainNameResponse"
    }
}
