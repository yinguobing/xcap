use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartRecordingRequest {
    pub name: ::std::string::String,
}

impl Default for StartRecordingRequest {
    fn default() -> Self {
        StartRecordingRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl crate::Message for StartRecordingRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartRecordingResponse {}

impl Default for StartRecordingResponse {
    fn default() -> Self {
        StartRecordingResponse {}
    }
}

impl crate::Message for StartRecordingResponse {}

pub struct StartRecording;
impl crate::Service for StartRecording {
    type Request = StartRecordingRequest;
    type Response = StartRecordingResponse;

    fn request_type_name(&self) -> &str {
        "StartRecordingRequest"
    }
    fn response_type_name(&self) -> &str {
        "StartRecordingResponse"
    }
}
