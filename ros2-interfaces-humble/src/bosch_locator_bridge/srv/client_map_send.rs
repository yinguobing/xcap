use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapSendRequest {
    pub name: ::std::string::String,
}

impl Default for ClientMapSendRequest {
    fn default() -> Self {
        ClientMapSendRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl crate::Message for ClientMapSendRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapSendResponse {}

impl Default for ClientMapSendResponse {
    fn default() -> Self {
        ClientMapSendResponse {}
    }
}

impl crate::Message for ClientMapSendResponse {}

pub struct ClientMapSend;
impl crate::Service for ClientMapSend {
    type Request = ClientMapSendRequest;
    type Response = ClientMapSendResponse;

    fn request_type_name(&self) -> &str {
        "ClientMapSendRequest"
    }
    fn response_type_name(&self) -> &str {
        "ClientMapSendResponse"
    }
}
