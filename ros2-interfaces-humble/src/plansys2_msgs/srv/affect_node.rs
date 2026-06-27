use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectNodeRequest {
    pub node: crate::plansys2_msgs::msg::Node,
}

impl Default for AffectNodeRequest {
    fn default() -> Self {
        AffectNodeRequest {
            node: crate::plansys2_msgs::msg::Node::default(),
        }
    }
}

impl crate::Message for AffectNodeRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectNodeResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AffectNodeResponse {
    fn default() -> Self {
        AffectNodeResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl crate::Message for AffectNodeResponse {}

pub struct AffectNode;
impl crate::Service for AffectNode {
    type Request = AffectNodeRequest;
    type Response = AffectNodeResponse;

    fn request_type_name(&self) -> &str {
        "AffectNodeRequest"
    }
    fn response_type_name(&self) -> &str {
        "AffectNodeResponse"
    }
}
