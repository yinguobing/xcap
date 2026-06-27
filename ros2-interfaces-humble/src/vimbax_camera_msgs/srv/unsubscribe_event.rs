use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeEventRequest {
    pub name: ::std::string::String,
}

impl Default for UnsubscribeEventRequest {
    fn default() -> Self {
        UnsubscribeEventRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl crate::Message for UnsubscribeEventRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeEventResponse {}

impl Default for UnsubscribeEventResponse {
    fn default() -> Self {
        UnsubscribeEventResponse {}
    }
}

impl crate::Message for UnsubscribeEventResponse {}

pub struct UnsubscribeEvent;
impl crate::Service for UnsubscribeEvent {
    type Request = UnsubscribeEventRequest;
    type Response = UnsubscribeEventResponse;

    fn request_type_name(&self) -> &str {
        "UnsubscribeEventRequest"
    }
    fn response_type_name(&self) -> &str {
        "UnsubscribeEventResponse"
    }
}
