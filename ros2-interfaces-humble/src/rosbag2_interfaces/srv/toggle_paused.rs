use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TogglePausedRequest {}

impl Default for TogglePausedRequest {
    fn default() -> Self {
        TogglePausedRequest {}
    }
}

impl crate::Message for TogglePausedRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TogglePausedResponse {}

impl Default for TogglePausedResponse {
    fn default() -> Self {
        TogglePausedResponse {}
    }
}

impl crate::Message for TogglePausedResponse {}

pub struct TogglePaused;
impl crate::Service for TogglePaused {
    type Request = TogglePausedRequest;
    type Response = TogglePausedResponse;

    fn request_type_name(&self) -> &str {
        "TogglePausedRequest"
    }
    fn response_type_name(&self) -> &str {
        "TogglePausedResponse"
    }
}
