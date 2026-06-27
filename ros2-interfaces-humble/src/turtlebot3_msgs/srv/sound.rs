use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundRequest {
    pub value: u8,
}

impl Default for SoundRequest {
    fn default() -> Self {
        SoundRequest { value: 0 }
    }
}

impl crate::Message for SoundRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SoundResponse {
    fn default() -> Self {
        SoundResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for SoundResponse {}

pub struct Sound;
impl crate::Service for Sound {
    type Request = SoundRequest;
    type Response = SoundResponse;

    fn request_type_name(&self) -> &str {
        "SoundRequest"
    }
    fn response_type_name(&self) -> &str {
        "SoundResponse"
    }
}
