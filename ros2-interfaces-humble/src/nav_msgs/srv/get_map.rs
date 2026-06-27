use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetMapRequest {}

impl crate::Message for GetMapRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetMapResponse {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
}

impl crate::Message for GetMapResponse {}

pub struct GetMap;
impl crate::Service for GetMap {
    type Request = GetMapRequest;
    type Response = GetMapResponse;

    fn request_type_name(&self) -> &str {
        "GetMapRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetMapResponse"
    }
}
