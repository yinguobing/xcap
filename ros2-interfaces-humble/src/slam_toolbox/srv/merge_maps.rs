use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergeMapsRequest {}

impl Default for MergeMapsRequest {
    fn default() -> Self {
        MergeMapsRequest {}
    }
}

impl crate::Message for MergeMapsRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergeMapsResponse {}

impl Default for MergeMapsResponse {
    fn default() -> Self {
        MergeMapsResponse {}
    }
}

impl crate::Message for MergeMapsResponse {}

pub struct MergeMaps;
impl crate::Service for MergeMaps {
    type Request = MergeMapsRequest;
    type Response = MergeMapsResponse;

    fn request_type_name(&self) -> &str {
        "MergeMapsRequest"
    }
    fn response_type_name(&self) -> &str {
        "MergeMapsResponse"
    }
}
