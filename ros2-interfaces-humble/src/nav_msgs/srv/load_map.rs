use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMapRequest {
    pub map_url: ::std::string::String,
}

impl crate::Message for LoadMapRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMapResponse {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub result: u8,
}

impl LoadMapResponse {
    pub const RESULT_SUCCESS: u8 = 0;
    pub const RESULT_MAP_DOES_NOT_EXIST: u8 = 1;
    pub const RESULT_INVALID_MAP_DATA: u8 = 2;
    pub const RESULT_INVALID_MAP_METADATA: u8 = 3;
    pub const RESULT_UNDEFINED_FAILURE: u8 = 255;
}

impl crate::Message for LoadMapResponse {}

pub struct LoadMap;
impl crate::Service for LoadMap {
    type Request = LoadMapRequest;
    type Response = LoadMapResponse;

    fn request_type_name(&self) -> &str {
        "LoadMapRequest"
    }
    fn response_type_name(&self) -> &str {
        "LoadMapResponse"
    }
}
