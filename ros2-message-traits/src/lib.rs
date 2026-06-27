use serde::{de::DeserializeOwned, Serialize};

/// Marker trait for ROS 2 message types.
///
/// Every ROS 2 message must be serializable and deserializable.
/// This is the same bound as [`ros2_client::Message`] — the two are drop-in compatible.
pub trait Message: Serialize + DeserializeOwned {}

/// Trait for ROS 2 service types.
///
/// A service pairs a request message with a response message.
/// This is the same bound as [`ros2_client::Service`].
pub trait Service {
    type Request: Message;
    type Response: Message;
    fn request_type_name(&self) -> &str;
    fn response_type_name(&self) -> &str;
}

// Blanket impls for common primitive message types
impl Message for String {}
impl Message for () {}
