pub mod msg {
    use serde::Deserialize;

    /// Time indicates a specific point in time, relative to a clock's 0 point.
    ///
    /// The seconds component, valid over all int32 values.
    /// int32 sec
    ///
    /// The nanoseconds component, valid in the range [0, 10e9).
    /// uint32 nanosec
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
    pub struct Time {
        pub sec: i32,
        pub nanosec: u32,
    }

    /// # Duration defines a period between two time points. It is comprised of a
    /// seconds component and a nanoseconds component.
    ///
    /// Seconds component, range is valid over any possible int32 value.
    /// int32 sec
    ///
    /// Nanoseconds component in the range of [0, 10e9).
    /// uint32 nanosec
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
    pub struct Duration {
        pub sec: i32,
        pub nanosec: u32,
    }
}
