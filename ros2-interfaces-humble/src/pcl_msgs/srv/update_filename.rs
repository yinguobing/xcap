use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameRequest {
    pub filename: ::std::string::String,
}

impl Default for UpdateFilenameRequest {
    fn default() -> Self {
        UpdateFilenameRequest {
            filename: ::std::string::String::new(),
        }
    }
}

impl crate::Message for UpdateFilenameRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameResponse {
    pub success: bool,
}

impl Default for UpdateFilenameResponse {
    fn default() -> Self {
        UpdateFilenameResponse { success: false }
    }
}

impl crate::Message for UpdateFilenameResponse {}

pub struct UpdateFilename;
impl crate::Service for UpdateFilename {
    type Request = UpdateFilenameRequest;
    type Response = UpdateFilenameResponse;

    fn request_type_name(&self) -> &str {
        "UpdateFilenameRequest"
    }
    fn response_type_name(&self) -> &str {
        "UpdateFilenameResponse"
    }
}
