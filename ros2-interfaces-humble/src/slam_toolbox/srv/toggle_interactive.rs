use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleInteractiveRequest {}

impl Default for ToggleInteractiveRequest {
    fn default() -> Self {
        ToggleInteractiveRequest {}
    }
}

impl crate::Message for ToggleInteractiveRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleInteractiveResponse {}

impl Default for ToggleInteractiveResponse {
    fn default() -> Self {
        ToggleInteractiveResponse {}
    }
}

impl crate::Message for ToggleInteractiveResponse {}

pub struct ToggleInteractive;
impl crate::Service for ToggleInteractive {
    type Request = ToggleInteractiveRequest;
    type Response = ToggleInteractiveResponse;

    fn request_type_name(&self) -> &str {
        "ToggleInteractiveRequest"
    }
    fn response_type_name(&self) -> &str {
        "ToggleInteractiveResponse"
    }
}
