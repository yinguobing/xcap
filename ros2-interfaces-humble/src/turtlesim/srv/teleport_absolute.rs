use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportAbsoluteRequest {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Default for TeleportAbsoluteRequest {
    fn default() -> Self {
        TeleportAbsoluteRequest {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
}

impl crate::Message for TeleportAbsoluteRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportAbsoluteResponse {}

impl Default for TeleportAbsoluteResponse {
    fn default() -> Self {
        TeleportAbsoluteResponse {}
    }
}

impl crate::Message for TeleportAbsoluteResponse {}

pub struct TeleportAbsolute;
impl crate::Service for TeleportAbsolute {
    type Request = TeleportAbsoluteRequest;
    type Response = TeleportAbsoluteResponse;

    fn request_type_name(&self) -> &str {
        "TeleportAbsoluteRequest"
    }
    fn response_type_name(&self) -> &str {
        "TeleportAbsoluteResponse"
    }
}
