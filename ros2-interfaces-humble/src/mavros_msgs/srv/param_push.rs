use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPushRequest {}

impl Default for ParamPushRequest {
    fn default() -> Self {
        ParamPushRequest {}
    }
}

impl crate::Message for ParamPushRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPushResponse {
    pub success: bool,
    pub param_transfered: u32,
}

impl Default for ParamPushResponse {
    fn default() -> Self {
        ParamPushResponse {
            success: false,
            param_transfered: 0,
        }
    }
}

impl crate::Message for ParamPushResponse {}

pub struct ParamPush;
impl crate::Service for ParamPush {
    type Request = ParamPushRequest;
    type Response = ParamPushResponse;

    fn request_type_name(&self) -> &str {
        "ParamPushRequest"
    }
    fn response_type_name(&self) -> &str {
        "ParamPushResponse"
    }
}
