use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionServersRequest {}

impl Default for GetActionServersRequest {
    fn default() -> Self {
        GetActionServersRequest {}
    }
}

impl crate::Message for GetActionServersRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionServersResponse {
    pub action_servers: Vec<::std::string::String>,
}

impl Default for GetActionServersResponse {
    fn default() -> Self {
        GetActionServersResponse {
            action_servers: Vec::new(),
        }
    }
}

impl crate::Message for GetActionServersResponse {}

pub struct GetActionServers;
impl crate::Service for GetActionServers {
    type Request = GetActionServersRequest;
    type Response = GetActionServersResponse;

    fn request_type_name(&self) -> &str {
        "GetActionServersRequest"
    }
    fn response_type_name(&self) -> &str {
        "GetActionServersResponse"
    }
}
