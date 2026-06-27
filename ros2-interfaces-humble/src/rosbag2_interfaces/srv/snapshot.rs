use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotRequest {}

impl Default for SnapshotRequest {
    fn default() -> Self {
        SnapshotRequest {}
    }
}

impl crate::Message for SnapshotRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotResponse {
    pub success: bool,
}

impl Default for SnapshotResponse {
    fn default() -> Self {
        SnapshotResponse { success: false }
    }
}

impl crate::Message for SnapshotResponse {}

pub struct Snapshot;
impl crate::Service for Snapshot {
    type Request = SnapshotRequest;
    type Response = SnapshotResponse;

    fn request_type_name(&self) -> &str {
        "SnapshotRequest"
    }
    fn response_type_name(&self) -> &str {
        "SnapshotResponse"
    }
}
