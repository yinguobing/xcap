use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionState {
    pub devices: Vec<crate::qb_device_msgs::msg::DeviceConnectionInfo>,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState {
            devices: Vec::new(),
        }
    }
}

impl crate::Message for ConnectionState {}
