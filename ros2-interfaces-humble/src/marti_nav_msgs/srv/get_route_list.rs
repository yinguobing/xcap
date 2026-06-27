use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteListRequest {}

impl Default for GetRouteListRequest {
    fn default() -> Self {
        GetRouteListRequest {}
    }
}

impl crate::Message for GetRouteListRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteListResponse {
    pub routes: Vec<crate::marti_nav_msgs::msg::Route>,
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetRouteListResponse {
    fn default() -> Self {
        GetRouteListResponse {
            routes: Vec::new(),
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for GetRouteListResponse {}

pub struct GetRouteList;
impl crate::Service for GetRouteList {
    type Request = GetRouteListRequest;
    type Response = GetRouteListResponse;

    fn request_type_name(&self) -> &str {
        "GetRouteListRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetRouteListResponse"
    }
}
