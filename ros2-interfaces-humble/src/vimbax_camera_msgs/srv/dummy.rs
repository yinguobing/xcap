use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DummyRequest {}

impl Default for DummyRequest {
    fn default() -> Self {
        DummyRequest {}
    }
}

impl crate::Message for DummyRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DummyResponse {}

impl Default for DummyResponse {
    fn default() -> Self {
        DummyResponse {}
    }
}

impl crate::Message for DummyResponse {}

pub struct Dummy;
impl crate::Service for Dummy {
    type Request = DummyRequest;
    type Response = DummyResponse;

    fn request_type_name(&self) -> &str {
        "DummyRequest"
    }
    fn response_type_name(&self) -> &str {
        "DummyResponse"
    }
}
