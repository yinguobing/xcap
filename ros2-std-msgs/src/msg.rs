use ros2_builtin_interfaces::msg::Time;
use serde::Deserialize;

/// Standard metadata for higher-level stamped data types.
/// This is generally used to communicate timestamped data
/// in a particular coordinate frame.
/// Two-integer timestamp that is expressed as seconds and nanoseconds.
/// builtin_interfaces/Time stamp
/// Transform frame with which this data is associated.
/// string frame_id
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Header {
    pub stamp: Time,
    pub frame_id: String,
}
