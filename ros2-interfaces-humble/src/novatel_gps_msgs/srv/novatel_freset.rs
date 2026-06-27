use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelFRESETRequest {
    pub target: ::std::string::String,
}

impl Default for NovatelFRESETRequest {
    fn default() -> Self {
        NovatelFRESETRequest {
            target: ::std::string::String::new(),
        }
    }
}

impl crate::Message for NovatelFRESETRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelFRESETResponse {
    pub success: bool,
}

impl Default for NovatelFRESETResponse {
    fn default() -> Self {
        NovatelFRESETResponse { success: false }
    }
}

impl crate::Message for NovatelFRESETResponse {}

pub struct NovatelFRESET;
impl crate::Service for NovatelFRESET {
    type Request = NovatelFRESETRequest;
    type Response = NovatelFRESETResponse;

    fn request_type_name(&self) -> &str {
        "NovatelFRESETRequest"
    }
    fn response_type_name(&self) -> &str {
        "NovatelFRESETResponse"
    }
}
