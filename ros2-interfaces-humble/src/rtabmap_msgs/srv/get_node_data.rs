use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDataRequest {
    pub ids: Vec<i32>,
    pub images: bool,
    pub scan: bool,
    pub grid: bool,
    pub user_data: bool,
}

impl Default for GetNodeDataRequest {
    fn default() -> Self {
        GetNodeDataRequest {
            ids: Vec::new(),
            images: false,
            scan: false,
            grid: false,
            user_data: false,
        }
    }
}

impl crate::Message for GetNodeDataRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDataResponse {
    pub data: Vec<crate::rtabmap_msgs::msg::Node>,
}

impl Default for GetNodeDataResponse {
    fn default() -> Self {
        GetNodeDataResponse { data: Vec::new() }
    }
}

impl crate::Message for GetNodeDataResponse {}

pub struct GetNodeData;
impl crate::Service for GetNodeData {
    type Request = GetNodeDataRequest;
    type Response = GetNodeDataResponse;

    fn request_type_name(&self) -> &str {
        "GetNodeDataRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetNodeDataResponse"
    }
}
