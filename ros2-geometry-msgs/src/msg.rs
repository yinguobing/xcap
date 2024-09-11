use ros2_std_msgs::msg::Header;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Accel {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AccelStamped {
    pub header: Header,
    pub accel: Accel,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Inertia {
    pub m: f64,
    pub com: Vector3,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct InertiaStamped {
    pub header: Header,
    pub inertia: Inertia,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct PointStamped {
    pub header: Header,
    pub point: Point,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Pose2D {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct TransformStamped {
    pub header: Header,
    pub child_frame_id: String,
    pub transform: Transform,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct TwistStamped {
    pub header: Header,
    pub twist: Twist,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
