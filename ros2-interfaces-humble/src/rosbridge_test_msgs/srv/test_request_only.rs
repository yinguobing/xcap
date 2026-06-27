use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestOnlyRequest {
    pub data: i32,
}

impl Default for TestRequestOnlyRequest {
    fn default() -> Self {
        TestRequestOnlyRequest { data: 0 }
    }
}

impl crate::Message for TestRequestOnlyRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestOnlyResponse {}

impl Default for TestRequestOnlyResponse {
    fn default() -> Self {
        TestRequestOnlyResponse {}
    }
}

impl crate::Message for TestRequestOnlyResponse {}

pub struct TestRequestOnly;
impl crate::Service for TestRequestOnly {
    type Request = TestRequestOnlyRequest;
    type Response = TestRequestOnlyResponse;

    fn request_type_name(&self) -> &str {
        "TestRequestOnlyRequest"
    }
    fn response_type_name(&self) -> &str {
        "TestRequestOnlyResponse"
    }
}
