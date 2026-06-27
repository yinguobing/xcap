use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSafetyModeRequest {}

impl Default for GetSafetyModeRequest {
    fn default() -> Self {
        GetSafetyModeRequest {}
    }
}

impl crate::Message for GetSafetyModeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSafetyModeResponse {
    pub safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetSafetyModeResponse {
    fn default() -> Self {
        GetSafetyModeResponse {
            safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode::default(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl crate::Message for GetSafetyModeResponse {}

pub struct GetSafetyMode;
impl crate::Service for GetSafetyMode {
    type Request = GetSafetyModeRequest;
    type Response = GetSafetyModeResponse;

    fn request_type_name(&self) -> &str {
        "GetSafetyModeRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetSafetyModeResponse"
    }
}
