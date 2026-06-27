use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemRequest {
    pub problem: ::std::string::String,
}

impl Default for AddProblemRequest {
    fn default() -> Self {
        AddProblemRequest {
            problem: ::std::string::String::new(),
        }
    }
}

impl crate::Message for AddProblemRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AddProblemResponse {
    fn default() -> Self {
        AddProblemResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl crate::Message for AddProblemResponse {}

pub struct AddProblem;
impl crate::Service for AddProblem {
    type Request = AddProblemRequest;
    type Response = AddProblemResponse;

    fn request_type_name(&self) -> &str {
        "AddProblemRequest"
    }
    fn response_type_name(&self) -> &str {
        "AddProblemResponse"
    }
}
