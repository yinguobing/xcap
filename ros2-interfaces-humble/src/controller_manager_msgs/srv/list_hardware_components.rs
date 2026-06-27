use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareComponentsRequest {}

impl Default for ListHardwareComponentsRequest {
    fn default() -> Self {
        ListHardwareComponentsRequest {}
    }
}

impl crate::Message for ListHardwareComponentsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareComponentsResponse {
    pub component: Vec<crate::controller_manager_msgs::msg::HardwareComponentState>,
}

impl Default for ListHardwareComponentsResponse {
    fn default() -> Self {
        ListHardwareComponentsResponse {
            component: Vec::new(),
        }
    }
}

impl crate::Message for ListHardwareComponentsResponse {}

pub struct ListHardwareComponents;
impl crate::Service for ListHardwareComponents {
    type Request = ListHardwareComponentsRequest;
    type Response = ListHardwareComponentsResponse;

    fn request_type_name(&self) -> &str {
        "ListHardwareComponentsRequest"
    }
    fn response_type_name(&self) -> &str {
        "ListHardwareComponentsResponse"
    }
}
