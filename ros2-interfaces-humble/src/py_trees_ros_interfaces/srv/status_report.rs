use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusReportRequest {}

impl Default for StatusReportRequest {
    fn default() -> Self {
        StatusReportRequest {}
    }
}

impl crate::Message for StatusReportRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusReportResponse {
    pub report: ::std::string::String,
}

impl Default for StatusReportResponse {
    fn default() -> Self {
        StatusReportResponse {
            report: ::std::string::String::new(),
        }
    }
}

impl crate::Message for StatusReportResponse {}

pub struct StatusReport;
impl crate::Service for StatusReport {
    type Request = StatusReportRequest;
    type Response = StatusReportResponse;

    fn request_type_name(&self) -> &str {
        "StatusReportRequest"
    }
    fn response_type_name(&self) -> &str {
        "StatusReportResponse"
    }
}
