use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectParamRequest {
    pub param: crate::plansys2_msgs::msg::Param,
}

impl Default for AffectParamRequest {
    fn default() -> Self {
        AffectParamRequest {
            param: crate::plansys2_msgs::msg::Param::default(),
        }
    }
}

impl crate::Message for AffectParamRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectParamResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AffectParamResponse {
    fn default() -> Self {
        AffectParamResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl crate::Message for AffectParamResponse {}

pub struct AffectParam;
impl crate::Service for AffectParam {
    type Request = AffectParamRequest;
    type Response = AffectParamResponse;

    fn request_type_name(&self) -> &str {
        "AffectParamRequest"
    }
    fn response_type_name(&self) -> &str {
        "AffectParamResponse"
    }
}
