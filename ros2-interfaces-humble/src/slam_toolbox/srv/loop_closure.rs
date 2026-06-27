use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopClosureRequest {}

impl Default for LoopClosureRequest {
    fn default() -> Self {
        LoopClosureRequest {}
    }
}

impl crate::Message for LoopClosureRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopClosureResponse {}

impl Default for LoopClosureResponse {
    fn default() -> Self {
        LoopClosureResponse {}
    }
}

impl crate::Message for LoopClosureResponse {}

pub struct LoopClosure;
impl crate::Service for LoopClosure {
    type Request = LoopClosureRequest;
    type Response = LoopClosureResponse;

    fn request_type_name(&self) -> &str {
        "LoopClosureRequest"
    }
    fn response_type_name(&self) -> &str {
        "LoopClosureResponse"
    }
}
