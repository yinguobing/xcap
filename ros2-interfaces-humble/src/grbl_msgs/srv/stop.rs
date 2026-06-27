use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRequest {
    pub machine_id: ::std::string::String,
    pub command: ::std::string::String,
}

impl Default for StopRequest {
    fn default() -> Self {
        StopRequest {
            machine_id: ::std::string::String::new(),
            command: ::std::string::String::new(),
        }
    }
}

impl crate::Message for StopRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopResponse {
    pub success: bool,
}

impl Default for StopResponse {
    fn default() -> Self {
        StopResponse { success: false }
    }
}

impl crate::Message for StopResponse {}

pub struct Stop;
impl crate::Service for Stop {
    type Request = StopRequest;
    type Response = StopResponse;

    fn request_type_name(&self) -> &str {
        "StopRequest"
    }
    fn response_type_name(&self) -> &str {
        "StopResponse"
    }
}
