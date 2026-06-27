use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MolaRuntimeParamSetRequest {
    pub parameters: ::std::string::String,
}

impl Default for MolaRuntimeParamSetRequest {
    fn default() -> Self {
        MolaRuntimeParamSetRequest {
            parameters: ::std::string::String::new(),
        }
    }
}

impl crate::Message for MolaRuntimeParamSetRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MolaRuntimeParamSetResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for MolaRuntimeParamSetResponse {
    fn default() -> Self {
        MolaRuntimeParamSetResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for MolaRuntimeParamSetResponse {}

pub struct MolaRuntimeParamSet;
impl crate::Service for MolaRuntimeParamSet {
    type Request = MolaRuntimeParamSetRequest;
    type Response = MolaRuntimeParamSetResponse;

    fn request_type_name(&self) -> &str {
        "MolaRuntimeParamSetRequest"
    }
    fn response_type_name(&self) -> &str {
        "MolaRuntimeParamSetResponse"
    }
}
