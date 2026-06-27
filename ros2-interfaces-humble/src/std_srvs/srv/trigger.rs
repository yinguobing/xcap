use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRequest {}

impl Default for TriggerRequest {
    fn default() -> Self {
        TriggerRequest {}
    }
}

impl crate::Message for TriggerRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerResponse {
    fn default() -> Self {
        TriggerResponse {
            success: false,
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
