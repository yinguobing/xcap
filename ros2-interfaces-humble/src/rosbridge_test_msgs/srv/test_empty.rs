use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEmptyRequest {}

impl Default for TestEmptyRequest {
    fn default() -> Self {
        TestEmptyRequest {}
    }
}

impl crate::Message for TestEmptyRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEmptyResponse {}

impl Default for TestEmptyResponse {
    fn default() -> Self {
        TestEmptyResponse {}
    }
}

impl crate::Message for TestEmptyResponse {}

pub struct TestEmpty;
impl crate::Service for TestEmpty {
    type Request = TestEmptyRequest;
    type Response = TestEmptyResponse;

    fn request_type_name(&self) -> &str {
        "TestEmptyRequest"
    }
    fn response_type_name(&self) -> &str {
        "TestEmptyResponse"
    }
}
