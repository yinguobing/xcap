use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserdataRequest {
    pub userdata_key: ::std::string::String,
}

impl Default for GetUserdataRequest {
    fn default() -> Self {
        GetUserdataRequest {
            userdata_key: ::std::string::String::new(),
        }
    }
}

impl crate::Message for GetUserdataRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserdataResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub userdata: Vec<crate::flexbe_msgs::msg::UserdataInfo>,
}

impl Default for GetUserdataResponse {
    fn default() -> Self {
        GetUserdataResponse {
            success: false,
            message: ::std::string::String::new(),
            userdata: Vec::new(),
        }
    }
}

impl crate::Message for GetUserdataResponse {}

pub struct GetUserdata;
impl crate::Service for GetUserdata {
    type Request = GetUserdataRequest;
    type Response = GetUserdataResponse;

    fn request_type_name(&self) -> &str {
        "GetUserdataRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetUserdataResponse"
    }
}
