use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFileListInfoRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetFileListInfoRequest {
    fn default() -> Self {
        CameraGetFileListInfoRequest { payload_index: 1 }
    }
}

impl crate::Message for CameraGetFileListInfoRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetFileListInfoResponse {
    pub success: bool,
    pub file_list: Vec<crate::psdk_interfaces::msg::FileInfo>,
    pub count: i16,
}

impl Default for CameraGetFileListInfoResponse {
    fn default() -> Self {
        CameraGetFileListInfoResponse {
            success: false,
            file_list: Vec::new(),
            count: 0,
        }
    }
}

impl crate::Message for CameraGetFileListInfoResponse {}

pub struct CameraGetFileListInfo;
impl crate::Service for CameraGetFileListInfo {
    type Request = CameraGetFileListInfoRequest;
    type Response = CameraGetFileListInfoResponse;

    fn request_type_name(&self) -> &str {
        "CameraGetFileListInfoRequest"
    }
    fn response_type_name(&self) -> &str {
        "CameraGetFileListInfoResponse"
    }
}
