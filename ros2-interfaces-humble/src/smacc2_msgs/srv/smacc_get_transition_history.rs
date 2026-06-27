use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccGetTransitionHistoryRequest {}

impl Default for SmaccGetTransitionHistoryRequest {
    fn default() -> Self {
        SmaccGetTransitionHistoryRequest {}
    }
}

impl crate::Message for SmaccGetTransitionHistoryRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccGetTransitionHistoryResponse {
    pub history: Vec<crate::smacc2_msgs::msg::SmaccTransitionLogEntry>,
}

impl Default for SmaccGetTransitionHistoryResponse {
    fn default() -> Self {
        SmaccGetTransitionHistoryResponse {
            history: Vec::new(),
        }
    }
}

impl crate::Message for SmaccGetTransitionHistoryResponse {}

pub struct SmaccGetTransitionHistory;
impl crate::Service for SmaccGetTransitionHistory {
    type Request = SmaccGetTransitionHistoryRequest;
    type Response = SmaccGetTransitionHistoryResponse;

    fn request_type_name(&self) -> &str {
        "SmaccGetTransitionHistoryRequest"
    }
    fn response_type_name(&self) -> &str {
        "SmaccGetTransitionHistoryResponse"
    }
}
