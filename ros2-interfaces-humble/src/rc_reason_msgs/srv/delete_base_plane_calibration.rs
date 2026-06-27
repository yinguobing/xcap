use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteBasePlaneCalibrationRequest {}

impl Default for DeleteBasePlaneCalibrationRequest {
    fn default() -> Self {
        DeleteBasePlaneCalibrationRequest {}
    }
}

impl crate::Message for DeleteBasePlaneCalibrationRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteBasePlaneCalibrationResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteBasePlaneCalibrationResponse {
    fn default() -> Self {
        DeleteBasePlaneCalibrationResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl crate::Message for DeleteBasePlaneCalibrationResponse {}

pub struct DeleteBasePlaneCalibration;
impl crate::Service for DeleteBasePlaneCalibration {
    type Request = DeleteBasePlaneCalibrationRequest;
    type Response = DeleteBasePlaneCalibrationResponse;

    fn request_type_name(&self) -> &str {
        "DeleteBasePlaneCalibrationRequest"
    }
    fn response_type_name(&self) -> &str {
        "DeleteBasePlaneCalibrationResponse"
    }
}
