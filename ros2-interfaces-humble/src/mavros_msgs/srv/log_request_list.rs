use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestListRequest {
    pub start: u16,
    pub end: u16,
}

impl Default for LogRequestListRequest {
    fn default() -> Self {
        LogRequestListRequest { start: 0, end: 0 }
    }
}

impl crate::Message for LogRequestListRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestListResponse {
    pub success: bool,
}

impl Default for LogRequestListResponse {
    fn default() -> Self {
        LogRequestListResponse { success: false }
    }
}

impl crate::Message for LogRequestListResponse {}

pub struct LogRequestList;
impl crate::Service for LogRequestList {
    type Request = LogRequestListRequest;
    type Response = LogRequestListResponse;

    fn request_type_name(&self) -> &str {
        "LogRequestListRequest"
    }
    fn response_type_name(&self) -> &str {
        "LogRequestListResponse"
    }
}
