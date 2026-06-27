use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionRequest {}

impl Default for GetRobotSoftwareVersionRequest {
    fn default() -> Self {
        GetRobotSoftwareVersionRequest {}
    }
}

impl crate::Message for GetRobotSoftwareVersionRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionResponse {
    pub major: u32,
    pub minor: u32,
    pub bugfix: u32,
    pub build: u32,
}

impl Default for GetRobotSoftwareVersionResponse {
    fn default() -> Self {
        GetRobotSoftwareVersionResponse {
            major: 0,
            minor: 0,
            bugfix: 0,
            build: 0,
        }
    }
}

impl crate::Message for GetRobotSoftwareVersionResponse {}

pub struct GetRobotSoftwareVersion;
impl crate::Service for GetRobotSoftwareVersion {
    type Request = GetRobotSoftwareVersionRequest;
    type Response = GetRobotSoftwareVersionResponse;

    fn request_type_name(&self) -> &str {
        "GetRobotSoftwareVersionRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetRobotSoftwareVersionResponse"
    }
}
