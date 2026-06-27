use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfTestRequest {}

impl crate::Message for SelfTestRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfTestResponse {
    pub id: ::std::string::String,
    pub passed: u8,
    pub status: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
}

impl crate::Message for SelfTestResponse {}

pub struct SelfTest;
impl crate::Service for SelfTest {
    type Request = SelfTestRequest;
    type Response = SelfTestResponse;

    fn request_type_name(&self) -> &str {
        "SelfTestRequest"
    }
    fn response_type_name(&self) -> &str {
        "SelfTestResponse"
    }
}
