use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggersRequest {
    pub created_after: i64,
}

impl Default for ListTriggersRequest {
    fn default() -> Self {
        ListTriggersRequest { created_after: 0 }
    }
}

impl crate::Message for ListTriggersRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTriggersResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub triggers: Vec<crate::rmf_scheduler_msgs::msg::Trigger>,
}

impl Default for ListTriggersResponse {
    fn default() -> Self {
        ListTriggersResponse {
            success: false,
            message: ::std::string::String::new(),
            triggers: Vec::new(),
        }
    }
}

impl crate::Message for ListTriggersResponse {}

pub struct ListTriggers;
impl crate::Service for ListTriggers {
    type Request = ListTriggersRequest;
    type Response = ListTriggersResponse;

    fn request_type_name(&self) -> &str {
        "ListTriggersRequest"
    }
    fn response_type_name(&self) -> &str {
        "ListTriggersResponse"
    }
}
