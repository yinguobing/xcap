use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptionStereoVisionSetupRequest {
    pub stereo_cameras_direction: ::std::string::String,
    pub start_stop: bool,
}

impl Default for PerceptionStereoVisionSetupRequest {
    fn default() -> Self {
        PerceptionStereoVisionSetupRequest {
            stereo_cameras_direction: ::std::string::String::new(),
            start_stop: false,
        }
    }
}

impl crate::Message for PerceptionStereoVisionSetupRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptionStereoVisionSetupResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for PerceptionStereoVisionSetupResponse {
    fn default() -> Self {
        PerceptionStereoVisionSetupResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl crate::Message for PerceptionStereoVisionSetupResponse {}

pub struct PerceptionStereoVisionSetup;
impl crate::Service for PerceptionStereoVisionSetup {
    type Request = PerceptionStereoVisionSetupRequest;
    type Response = PerceptionStereoVisionSetupResponse;

    fn request_type_name(&self) -> &str {
        "PerceptionStereoVisionSetupRequest"
    }
    fn response_type_name(&self) -> &str {
        "PerceptionStereoVisionSetupResponse"
    }
}
