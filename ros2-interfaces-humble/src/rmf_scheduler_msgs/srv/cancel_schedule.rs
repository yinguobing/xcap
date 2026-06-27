use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelScheduleRequest {
    pub name: ::std::string::String,
    pub finished: bool,
}

impl Default for CancelScheduleRequest {
    fn default() -> Self {
        CancelScheduleRequest {
            name: ::std::string::String::new(),
            finished: false,
        }
    }
}

impl crate::Message for CancelScheduleRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelScheduleResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelScheduleResponse {
    fn default() -> Self {
        CancelScheduleResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for CancelScheduleResponse {}

pub struct CancelSchedule;
impl crate::Service for CancelSchedule {
    type Request = CancelScheduleRequest;
    type Response = CancelScheduleResponse;

    fn request_type_name(&self) -> &str {
        "CancelScheduleRequest"
    }
    fn response_type_name(&self) -> &str {
        "CancelScheduleResponse"
    }
}
