use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesRequest {}

impl Default for GetParamNamesRequest {
    fn default() -> Self {
        GetParamNamesRequest {}
    }
}

impl crate::Message for GetParamNamesRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesResponse {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParamNamesResponse {
    fn default() -> Self {
        GetParamNamesResponse { names: Vec::new() }
    }
}

impl crate::Message for GetParamNamesResponse {}

pub struct GetParamNames;
impl crate::Service for GetParamNames {
    type Request = GetParamNamesRequest;
    type Response = GetParamNamesResponse;

    fn request_type_name(&self) -> &str {
        "GetParamNamesRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetParamNamesResponse"
    }
}
