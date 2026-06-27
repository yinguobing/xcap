use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest3DRequest {
    pub region_of_interest_ids: Vec<::std::string::String>,
}

impl Default for DeleteRegionsOfInterest3DRequest {
    fn default() -> Self {
        DeleteRegionsOfInterest3DRequest {
            region_of_interest_ids: Vec::new(),
        }
    }
}

impl crate::Message for DeleteRegionsOfInterest3DRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest3DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteRegionsOfInterest3DResponse {
    fn default() -> Self {
        DeleteRegionsOfInterest3DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl crate::Message for DeleteRegionsOfInterest3DResponse {}

pub struct DeleteRegionsOfInterest3D;
impl crate::Service for DeleteRegionsOfInterest3D {
    type Request = DeleteRegionsOfInterest3DRequest;
    type Response = DeleteRegionsOfInterest3DResponse;

    fn request_type_name(&self) -> &str {
        "DeleteRegionsOfInterest3DRequest"
    }
    fn response_type_name(&self) -> &str {
        "DeleteRegionsOfInterest3DResponse"
    }
}
