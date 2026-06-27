use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOctomapRequest {}

impl Default for GetOctomapRequest {
    fn default() -> Self {
        GetOctomapRequest {}
    }
}

impl crate::Message for GetOctomapRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOctomapResponse {
    pub map: crate::octomap_msgs::msg::Octomap,
}

impl Default for GetOctomapResponse {
    fn default() -> Self {
        GetOctomapResponse {
            map: crate::octomap_msgs::msg::Octomap::default(),
        }
    }
}

impl crate::Message for GetOctomapResponse {}

pub struct GetOctomap;
impl crate::Service for GetOctomap {
    type Request = GetOctomapRequest;
    type Response = GetOctomapResponse;

    fn request_type_name(&self) -> &str {
        "GetOctomapRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetOctomapResponse"
    }
}
