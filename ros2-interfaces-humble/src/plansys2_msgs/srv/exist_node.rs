use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExistNodeRequest {
    pub node: crate::plansys2_msgs::msg::Node,
}

impl Default for ExistNodeRequest {
    fn default() -> Self {
        ExistNodeRequest {
            node: crate::plansys2_msgs::msg::Node::default(),
        }
    }
}

impl crate::Message for ExistNodeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExistNodeResponse {
    pub exist: bool,
}

impl Default for ExistNodeResponse {
    fn default() -> Self {
        ExistNodeResponse { exist: false }
    }
}

impl crate::Message for ExistNodeResponse {}

pub struct ExistNode;
impl crate::Service for ExistNode {
    type Request = ExistNodeRequest;
    type Response = ExistNodeResponse;

    fn request_type_name(&self) -> &str {
        "ExistNodeRequest"
    }
    fn response_type_name(&self) -> &str {
        "ExistNodeResponse"
    }
}
