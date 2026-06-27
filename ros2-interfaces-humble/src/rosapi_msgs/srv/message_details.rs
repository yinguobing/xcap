use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDetailsRequest {
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}

impl Default for MessageDetailsRequest {
    fn default() -> Self {
        MessageDetailsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

impl crate::Message for MessageDetailsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for MessageDetailsResponse {
    fn default() -> Self {
        MessageDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

impl crate::Message for MessageDetailsResponse {}

pub struct MessageDetails;
impl crate::Service for MessageDetails {
    type Request = MessageDetailsRequest;
    type Response = MessageDetailsResponse;

    fn request_type_name(&self) -> &str {
        "MessageDetailsRequest"
    }
    fn response_type_name(&self) -> &str {
        "MessageDetailsResponse"
    }
}
