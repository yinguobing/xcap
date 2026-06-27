use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayNextRequest {}

impl Default for PlayNextRequest {
    fn default() -> Self {
        PlayNextRequest {}
    }
}

impl crate::Message for PlayNextRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayNextResponse {
    pub success: bool,
}

impl Default for PlayNextResponse {
    fn default() -> Self {
        PlayNextResponse { success: false }
    }
}

impl crate::Message for PlayNextResponse {}

pub struct PlayNext;
impl crate::Service for PlayNext {
    type Request = PlayNextRequest;
    type Response = PlayNextResponse;

    fn request_type_name(&self) -> &str {
        "PlayNextRequest"
    }
    fn response_type_name(&self) -> &str {
        "PlayNextResponse"
    }
}
