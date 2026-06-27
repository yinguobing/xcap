use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBoolRequest {
    pub value: bool,
}

impl Default for CommandBoolRequest {
    fn default() -> Self {
        CommandBoolRequest { value: false }
    }
}

impl crate::Message for CommandBoolRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBoolResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandBoolResponse {
    fn default() -> Self {
        CommandBoolResponse {
            success: false,
            result: 0,
        }
    }
}

impl crate::Message for CommandBoolResponse {}

pub struct CommandBool;
impl crate::Service for CommandBool {
    type Request = CommandBoolRequest;
    type Response = CommandBoolResponse;

    fn request_type_name(&self) -> &str {
        "CommandBoolRequest"
    }
    fn response_type_name(&self) -> &str {
        "CommandBoolResponse"
    }
}
