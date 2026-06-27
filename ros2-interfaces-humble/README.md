# ROS2 Interfaces for Rust

This crate provides Rust structs for all interfaces (i.e., messages and services) listed on the
[ROS Index for Humble](https://index.ros.org/packages/#humble).

The `Message` and `Service` traits are defined locally, compatible with `ros2-client`'s traits.
You can use this crate standalone (for serialization/deserialization) or with `ros2-client` for
live ROS 2 communication.

Every ROS package is a separate Cargo feature — cherry-pick the interfaces you need, e.g. `std_msgs`,
`geometry_msgs`, `nav2_msgs`.

## Usage

```toml
[dependencies]
ros2-interfaces-humble = { version = "0.0.1", features = ["std_msgs", "sensor_msgs"] }
```

```rust
use ros2_interfaces_humble::std_msgs::msg;

let header = msg::Header {
    stamp: ros2_interfaces_humble::builtin_interfaces::msg::Time { sec: 0, nanosec: 0 },
    frame_id: "map".into(),
};
```

For ROS 2 communication, add `ros2-client` separately — this crate's `Message` and `Service` traits
are drop-in compatible with `ros2_client::Message` and `ros2_client::Service`.

## Known Issues

The following packages do not compile and their features are disabled:

* `depthai_ros_msgs`
* `mrpt_msgs`
* `mrpt_nav_interfaces`
* `rosbag2_storage_mcap_testdata`
* `sick_scan_xd`
* `ublox_msgs`
