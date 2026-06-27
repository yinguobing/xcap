use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRouteRequest {
    pub guid: ::std::string::String,
}

impl Default for DeleteRouteRequest {
    fn default() -> Self {
        DeleteRouteRequest {
            guid: ::std::string::String::new(),
        }
    }
}

impl crate::Message for DeleteRouteRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for DeleteRouteResponse {
    fn default() -> Self {
        DeleteRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for DeleteRouteResponse {}

pub struct DeleteRoute;
impl crate::Service for DeleteRoute {
    type Request = DeleteRouteRequest;
    type Response = DeleteRouteResponse;

    fn request_type_name(&self) -> &str {
        "DeleteRouteRequest"
    }
    fn response_type_name(&self) -> &str {
        "DeleteRouteResponse"
    }
}
