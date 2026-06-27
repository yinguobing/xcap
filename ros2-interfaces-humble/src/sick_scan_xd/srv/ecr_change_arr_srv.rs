use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvRequest {
    pub active: bool,
}

impl Default for ECRChangeArrSrvRequest {
    fn default() -> Self {
        ECRChangeArrSrvRequest {
            active: false,
        }
    }
}

impl crate::Message for ECRChangeArrSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvResponse {
    pub success: bool,
}

impl Default for ECRChangeArrSrvResponse {
    fn default() -> Self {
        ECRChangeArrSrvResponse {
            success: false,
        }
    }
}

impl crate::Message for ECRChangeArrSrvResponse {}


pub struct ECRChangeArrSrv;
impl crate::Service for ECRChangeArrSrv {
    type Request = ECRChangeArrSrvRequest;
    type Response = ECRChangeArrSrvResponse;

    fn request_type_name(&self) -> &str { "ECRChangeArrSrvRequest" }
    fn response_type_name(&self) -> &str { "ECRChangeArrSrvResponse" }
}
