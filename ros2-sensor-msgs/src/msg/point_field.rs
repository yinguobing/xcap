use serde::Deserialize;
/// This message holds a collection of N-dimensional points, which may
/// contain additional information such as normals, intensity, etc. The
///
/// point data is stored as a binary blob, its layout described by the
/// contents of the "fields" array.
///
/// The point cloud data may be organized 2d (image-like) or 1d (unordered).
///
/// Point clouds organized as 2d images may be produced by camera depth sensors
/// such as stereo or time-of-flight.
///
///
/// Time of sensor data acquisition, and the coordinate frame ID (for 3d points).
///
/// std_msgs/Header header
///
/// 2D structure of the point cloud. If the cloud is unordered, height is
/// 1 and width is the length of the point cloud.
///
/// uint32 height
/// uint32 width
///
/// Describes the channels and their layout in the binary data blob.
///
/// PointField[] fields
///
/// bool is_bigendian # Is this data bigendian?
///
/// uint32 point_step # Length of a point in bytes
///
/// uint32 row_step # Length of a row in bytes
///
/// uint8[] data # Actual point data, size is (row_step*height)
///
///
/// bool is_dense # True if there are no invalid points
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[allow(non_snake_case)]
pub struct PointField {
    INT8: u8,
    UINT8: u8,
    INT16: u8,
    UINT16: u8,
    INT32: u8,
    UINT32: u8,
    FLOAT32: u8,
    FLOAT64: u8,
    name: String,
    offset: u32,
    datatype: u8,
    count: u32,
}

impl Default for PointField {
    fn default() -> Self {
        PointField {
            INT8: 1,
            UINT8: 2,
            INT16: 3,
            UINT16: 4,
            INT32: 5,
            UINT32: 6,
            FLOAT32: 7,
            FLOAT64: 8,
            name: "".to_string(),
            offset: 0,
            datatype: 0,
            count: 0,
        }
    }
}
