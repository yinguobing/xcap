use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRobotStateFromWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for DeleteRobotStateFromWarehouseRequest {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl crate::Message for DeleteRobotStateFromWarehouseRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRobotStateFromWarehouseResponse {}

impl Default for DeleteRobotStateFromWarehouseResponse {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseResponse {}
    }
}

impl crate::Message for DeleteRobotStateFromWarehouseResponse {}

pub struct DeleteRobotStateFromWarehouse;
impl crate::Service for DeleteRobotStateFromWarehouse {
    type Request = DeleteRobotStateFromWarehouseRequest;
    type Response = DeleteRobotStateFromWarehouseResponse;

    fn request_type_name(&self) -> &str {
        "DeleteRobotStateFromWarehouseRequest"
    }
    fn response_type_name(&self) -> &str {
        "DeleteRobotStateFromWarehouseResponse"
    }
}
