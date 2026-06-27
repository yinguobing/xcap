use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesRequest {}

impl Default for GetAvailableStatesRequest {
    fn default() -> Self {
        GetAvailableStatesRequest {}
    }
}

impl crate::Message for GetAvailableStatesRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesResponse {
    pub available_states: Vec<crate::lifecycle_msgs::msg::State>,
}

impl Default for GetAvailableStatesResponse {
    fn default() -> Self {
        GetAvailableStatesResponse {
            available_states: Vec::new(),
        }
    }
}

impl crate::Message for GetAvailableStatesResponse {}

pub struct GetAvailableStates;
impl crate::Service for GetAvailableStates {
    type Request = GetAvailableStatesRequest;
    type Response = GetAvailableStatesResponse;

    fn request_type_name(&self) -> &str {
        "GetAvailableStatesRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetAvailableStatesResponse"
    }
}
