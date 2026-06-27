use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AddDiagnosticsRequest {
    pub load_namespace: ::std::string::String,
}

impl crate::Message for AddDiagnosticsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AddDiagnosticsResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl crate::Message for AddDiagnosticsResponse {}

pub struct AddDiagnostics;
impl crate::Service for AddDiagnostics {
    type Request = AddDiagnosticsRequest;
    type Response = AddDiagnosticsResponse;

    fn request_type_name(&self) -> &str {
        "AddDiagnosticsRequest"
    }
    fn response_type_name(&self) -> &str {
        "AddDiagnosticsResponse"
    }
}
