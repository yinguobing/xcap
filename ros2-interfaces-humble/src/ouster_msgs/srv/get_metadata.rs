use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetadataRequest {}

impl Default for GetMetadataRequest {
    fn default() -> Self {
        GetMetadataRequest {}
    }
}

impl crate::Message for GetMetadataRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetadataResponse {
    pub metadata: crate::ouster_msgs::msg::Metadata,
}

impl Default for GetMetadataResponse {
    fn default() -> Self {
        GetMetadataResponse {
            metadata: crate::ouster_msgs::msg::Metadata::default(),
        }
    }
}

impl crate::Message for GetMetadataResponse {}

pub struct GetMetadata;
impl crate::Service for GetMetadata {
    type Request = GetMetadataRequest;
    type Response = GetMetadataResponse;

    fn request_type_name(&self) -> &str {
        "GetMetadataRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetMetadataResponse"
    }
}
