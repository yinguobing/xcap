use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadedProgramRequest {}

impl Default for GetLoadedProgramRequest {
    fn default() -> Self {
        GetLoadedProgramRequest {}
    }
}

impl crate::Message for GetLoadedProgramRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadedProgramResponse {
    pub answer: ::std::string::String,
    pub program_name: ::std::string::String,
    pub success: bool,
}

impl Default for GetLoadedProgramResponse {
    fn default() -> Self {
        GetLoadedProgramResponse {
            answer: ::std::string::String::new(),
            program_name: ::std::string::String::new(),
            success: false,
        }
    }
}

impl crate::Message for GetLoadedProgramResponse {}

pub struct GetLoadedProgram;
impl crate::Service for GetLoadedProgram {
    type Request = GetLoadedProgramRequest;
    type Response = GetLoadedProgramResponse;

    fn request_type_name(&self) -> &str {
        "GetLoadedProgramRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetLoadedProgramResponse"
    }
}
