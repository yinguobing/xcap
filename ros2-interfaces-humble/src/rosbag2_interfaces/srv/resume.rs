use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResumeRequest {}

impl Default for ResumeRequest {
    fn default() -> Self {
        ResumeRequest {}
    }
}

impl crate::Message for ResumeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResumeResponse {}

impl Default for ResumeResponse {
    fn default() -> Self {
        ResumeResponse {}
    }
}

impl crate::Message for ResumeResponse {}

pub struct Resume;
impl crate::Service for Resume {
    type Request = ResumeRequest;
    type Response = ResumeResponse;

    fn request_type_name(&self) -> &str {
        "ResumeRequest"
    }
    fn response_type_name(&self) -> &str {
        "ResumeResponse"
    }
}
