use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartGvmRecordingRequest {
    pub disable_window: bool,
    pub world_as_main_view: bool,
}

impl Default for StartGvmRecordingRequest {
    fn default() -> Self {
        StartGvmRecordingRequest {
            disable_window: false,
            world_as_main_view: false,
        }
    }
}

impl crate::Message for StartGvmRecordingRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartGvmRecordingResponse {}

impl Default for StartGvmRecordingResponse {
    fn default() -> Self {
        StartGvmRecordingResponse {}
    }
}

impl crate::Message for StartGvmRecordingResponse {}

pub struct StartGvmRecording;
impl crate::Service for StartGvmRecording {
    type Request = StartGvmRecordingRequest;
    type Response = StartGvmRecordingResponse;

    fn request_type_name(&self) -> &str {
        "StartGvmRecordingRequest"
    }
    fn response_type_name(&self) -> &str {
        "StartGvmRecordingResponse"
    }
}
