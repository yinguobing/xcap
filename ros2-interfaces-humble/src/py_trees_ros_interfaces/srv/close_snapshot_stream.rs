use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseSnapshotStreamRequest {
    pub topic_name: ::std::string::String,
}

impl Default for CloseSnapshotStreamRequest {
    fn default() -> Self {
        CloseSnapshotStreamRequest {
            topic_name: ::std::string::String::new(),
        }
    }
}

impl crate::Message for CloseSnapshotStreamRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseSnapshotStreamResponse {
    pub result: bool,
}

impl Default for CloseSnapshotStreamResponse {
    fn default() -> Self {
        CloseSnapshotStreamResponse { result: false }
    }
}

impl crate::Message for CloseSnapshotStreamResponse {}

pub struct CloseSnapshotStream;
impl crate::Service for CloseSnapshotStream {
    type Request = CloseSnapshotStreamRequest;
    type Response = CloseSnapshotStreamResponse;

    fn request_type_name(&self) -> &str {
        "CloseSnapshotStreamRequest"
    }
    fn response_type_name(&self) -> &str {
        "CloseSnapshotStreamResponse"
    }
}
