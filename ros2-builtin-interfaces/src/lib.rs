//! # ros2-builtin-interfaces
//!
//! Message definitions for types in the OMG IDL Platform Specific Model
//!
pub mod msg {
    use serde::{Deserialize, Serialize};

    /// Time indicates a specific point in time, relative to a clock's 0 point.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    pub struct Time {
        /// The seconds component, valid over all int32 values.
        pub sec: i32,

        /// The nanoseconds component, valid in the range [0, 10e9).
        pub nanosec: u32,
    }

    /// Duration defines a period between two time points. It is comprised of a seconds component and a nanoseconds component.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    pub struct Duration {
        /// Seconds component, range is valid over any possible int32 value.
        pub sec: i32,

        /// Nanoseconds component in the range of [0, 10e9).
        pub nanosec: u32,
    }
}
