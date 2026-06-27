use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapRequest {
    pub filename: ::std::string::String,
}

impl Default for LoadMapRequest {
    fn default() -> Self {
        LoadMapRequest {
            filename: ::std::string::String::new(),
        }
    }
}

impl crate::Message for LoadMapRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadMapResponse {
    pub success: bool,
}

impl Default for LoadMapResponse {
    fn default() -> Self {
        LoadMapResponse { success: false }
    }
}

impl crate::Message for LoadMapResponse {}

pub struct LoadMap;
impl crate::Service for LoadMap {
    type Request = LoadMapRequest;
    type Response = LoadMapResponse;

    fn request_type_name(&self) -> &str {
        "LoadMapRequest"
    }
    fn response_type_name(&self) -> &str {
        "LoadMapResponse"
    }
}
