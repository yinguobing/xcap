use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListNodesRequest {}

impl Default for ListNodesRequest {
    fn default() -> Self {
        ListNodesRequest {}
    }
}

impl crate::Message for ListNodesRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListNodesResponse {
    pub full_node_names: Vec<::std::string::String>,
    pub unique_ids: Vec<u64>,
}

impl Default for ListNodesResponse {
    fn default() -> Self {
        ListNodesResponse {
            full_node_names: Vec::new(),
            unique_ids: Vec::new(),
        }
    }
}

impl crate::Message for ListNodesResponse {}

pub struct ListNodes;
impl crate::Service for ListNodes {
    type Request = ListNodesRequest;
    type Response = ListNodesResponse;

    fn request_type_name(&self) -> &str {
        "ListNodesRequest"
    }
    fn response_type_name(&self) -> &str {
        "ListNodesResponse"
    }
}
