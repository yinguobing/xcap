use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcGapAllRequest {
    pub motor_num: u8,
}

impl Default for TmcGapAllRequest {
    fn default() -> Self {
        TmcGapAllRequest { motor_num: 0 }
    }
}

impl crate::Message for TmcGapAllRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcGapAllResponse {
    pub success: bool,
    pub param: Vec<crate::adi_tmcl::msg::TmcParam>,
}

impl Default for TmcGapAllResponse {
    fn default() -> Self {
        TmcGapAllResponse {
            success: false,
            param: Vec::new(),
        }
    }
}

impl crate::Message for TmcGapAllResponse {}

pub struct TmcGapAll;
impl crate::Service for TmcGapAll {
    type Request = TmcGapAllRequest;
    type Response = TmcGapAllResponse;

    fn request_type_name(&self) -> &str {
        "TmcGapAllRequest"
    }
    fn response_type_name(&self) -> &str {
        "TmcGapAllResponse"
    }
}
