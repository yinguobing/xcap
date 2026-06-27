use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlWorldRequest {
    pub world_control: crate::ros_gz_interfaces::msg::WorldControl,
}

impl Default for ControlWorldRequest {
    fn default() -> Self {
        ControlWorldRequest {
            world_control: crate::ros_gz_interfaces::msg::WorldControl::default(),
        }
    }
}

impl crate::Message for ControlWorldRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlWorldResponse {
    pub success: bool,
}

impl Default for ControlWorldResponse {
    fn default() -> Self {
        ControlWorldResponse { success: false }
    }
}

impl crate::Message for ControlWorldResponse {}

pub struct ControlWorld;
impl crate::Service for ControlWorld {
    type Request = ControlWorldRequest;
    type Response = ControlWorldResponse;

    fn request_type_name(&self) -> &str {
        "ControlWorldRequest"
    }
    fn response_type_name(&self) -> &str {
        "ControlWorldResponse"
    }
}
