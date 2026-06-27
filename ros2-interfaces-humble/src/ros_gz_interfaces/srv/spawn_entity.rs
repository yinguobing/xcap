use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityRequest {
    pub entity_factory: crate::ros_gz_interfaces::msg::EntityFactory,
}

impl Default for SpawnEntityRequest {
    fn default() -> Self {
        SpawnEntityRequest {
            entity_factory: crate::ros_gz_interfaces::msg::EntityFactory::default(),
        }
    }
}

impl crate::Message for SpawnEntityRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityResponse {
    pub success: bool,
}

impl Default for SpawnEntityResponse {
    fn default() -> Self {
        SpawnEntityResponse { success: false }
    }
}

impl crate::Message for SpawnEntityResponse {}

pub struct SpawnEntity;
impl crate::Service for SpawnEntity {
    type Request = SpawnEntityRequest;
    type Response = SpawnEntityResponse;

    fn request_type_name(&self) -> &str {
        "SpawnEntityRequest"
    }
    fn response_type_name(&self) -> &str {
        "SpawnEntityResponse"
    }
}
