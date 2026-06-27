use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearQueueRequest {}

impl Default for ClearQueueRequest {
    fn default() -> Self {
        ClearQueueRequest {}
    }
}

impl crate::Message for ClearQueueRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearQueueResponse {
    pub status: bool,
}

impl Default for ClearQueueResponse {
    fn default() -> Self {
        ClearQueueResponse { status: false }
    }
}

impl crate::Message for ClearQueueResponse {}

pub struct ClearQueue;
impl crate::Service for ClearQueue {
    type Request = ClearQueueRequest;
    type Response = ClearQueueResponse;

    fn request_type_name(&self) -> &str {
        "ClearQueueRequest"
    }
    fn response_type_name(&self) -> &str {
        "ClearQueueResponse"
    }
}
