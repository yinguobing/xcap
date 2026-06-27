use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramRunningRequest {}

impl Default for IsProgramRunningRequest {
    fn default() -> Self {
        IsProgramRunningRequest {}
    }
}

impl crate::Message for IsProgramRunningRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramRunningResponse {
    pub answer: ::std::string::String,
    pub program_running: bool,
    pub success: bool,
}

impl Default for IsProgramRunningResponse {
    fn default() -> Self {
        IsProgramRunningResponse {
            answer: ::std::string::String::new(),
            program_running: false,
            success: false,
        }
    }
}

impl crate::Message for IsProgramRunningResponse {}

pub struct IsProgramRunning;
impl crate::Service for IsProgramRunning {
    type Request = IsProgramRunningRequest;
    type Response = IsProgramRunningResponse;

    fn request_type_name(&self) -> &str {
        "IsProgramRunningRequest"
    }
    fn response_type_name(&self) -> &str {
        "IsProgramRunningResponse"
    }
}
