use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileTruncateRequest {
    pub file_path: ::std::string::String,
    pub length: u64,
}

impl Default for FileTruncateRequest {
    fn default() -> Self {
        FileTruncateRequest {
            file_path: ::std::string::String::new(),
            length: 0,
        }
    }
}

impl crate::Message for FileTruncateRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileTruncateResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileTruncateResponse {
    fn default() -> Self {
        FileTruncateResponse {
            success: false,
            r_errno: 0,
        }
    }
}

impl crate::Message for FileTruncateResponse {}

pub struct FileTruncate;
impl crate::Service for FileTruncate {
    type Request = FileTruncateRequest;
    type Response = FileTruncateResponse;

    fn request_type_name(&self) -> &str {
        "FileTruncateRequest"
    }
    fn response_type_name(&self) -> &str {
        "FileTruncateResponse"
    }
}
