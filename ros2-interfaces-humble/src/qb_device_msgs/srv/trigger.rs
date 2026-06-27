use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRequest {
    pub id: i32,
    pub max_repeats: i32,
}

impl Default for TriggerRequest {
    fn default() -> Self {
        TriggerRequest {
            id: 0,
            max_repeats: 0,
        }
    }
}

impl crate::Message for TriggerRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerResponse {
    pub success: bool,
    pub failures: i32,
    pub message: ::std::string::String,
}

impl Default for TriggerResponse {
    fn default() -> Self {
        TriggerResponse {
            success: false,
            failures: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for TriggerResponse {}

pub struct Trigger;
impl crate::Service for Trigger {
    type Request = TriggerRequest;
    type Response = TriggerResponse;

    fn request_type_name(&self) -> &str {
        "TriggerRequest"
    }
    fn response_type_name(&self) -> &str {
        "TriggerResponse"
    }
}
