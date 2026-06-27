use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupLocalGridsRequest {
    pub radius: i32,
    pub filter_scans: bool,
}

impl Default for CleanupLocalGridsRequest {
    fn default() -> Self {
        CleanupLocalGridsRequest {
            radius: 0,
            filter_scans: false,
        }
    }
}

impl crate::Message for CleanupLocalGridsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupLocalGridsResponse {
    pub modified: i32,
}

impl Default for CleanupLocalGridsResponse {
    fn default() -> Self {
        CleanupLocalGridsResponse { modified: 0 }
    }
}

impl crate::Message for CleanupLocalGridsResponse {}

pub struct CleanupLocalGrids;
impl crate::Service for CleanupLocalGrids {
    type Request = CleanupLocalGridsRequest;
    type Response = CleanupLocalGridsResponse;

    fn request_type_name(&self) -> &str {
        "CleanupLocalGridsRequest"
    }
    fn response_type_name(&self) -> &str {
        "CleanupLocalGridsResponse"
    }
}
