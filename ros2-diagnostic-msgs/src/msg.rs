use ros2_std_msgs::msg::Header;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

// This message holds the status of an individual component of the robot.
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[allow(non_snake_case)]
pub struct DiagnosticStatus {
    // Possible levels of operations.
    pub OK: u8,
    pub WARN: u8,
    pub ERROR: u8,
    pub STALE: u8,

    // Level of operation enumerated above.
    pub level: u8,
    // A description of the test/component reporting.
    pub name: String,
    // A description of the status.
    pub message: String,
    // A hardware unique string.
    pub hardware_id: String,
    // An array of values associated with the status.
    pub values: Vec<KeyValue>,
}

// This message is used to send diagnostic information about the state of the robot.
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct DiagnosticArray {
    // for timestamp
    pub header: Header,
    // an array of components being reported on
    pub status: Vec<DiagnosticStatus>,
}
