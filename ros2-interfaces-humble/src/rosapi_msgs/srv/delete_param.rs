use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteParamRequest {
    pub name: ::std::string::String,
}

impl Default for DeleteParamRequest {
    fn default() -> Self {
        DeleteParamRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl crate::Message for DeleteParamRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteParamResponse {}

impl Default for DeleteParamResponse {
    fn default() -> Self {
        DeleteParamResponse {}
    }
}

impl crate::Message for DeleteParamResponse {}

pub struct DeleteParam;
impl crate::Service for DeleteParam {
    type Request = DeleteParamRequest;
    type Response = DeleteParamResponse;

    fn request_type_name(&self) -> &str {
        "DeleteParamRequest"
    }
    fn response_type_name(&self) -> &str {
        "DeleteParamResponse"
    }
}
