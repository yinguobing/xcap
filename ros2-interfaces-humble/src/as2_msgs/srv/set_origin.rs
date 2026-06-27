use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetOriginRequest {
    pub origin: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for SetOriginRequest {
    fn default() -> Self {
        SetOriginRequest {
            origin: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

impl crate::Message for SetOriginRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetOriginResponse {
    pub success: bool,
}

impl Default for SetOriginResponse {
    fn default() -> Self {
        SetOriginResponse { success: false }
    }
}

impl crate::Message for SetOriginResponse {}

pub struct SetOrigin;
impl crate::Service for SetOrigin {
    type Request = SetOriginRequest;
    type Response = SetOriginResponse;

    fn request_type_name(&self) -> &str {
        "SetOriginRequest"
    }
    fn response_type_name(&self) -> &str {
        "SetOriginResponse"
    }
}
