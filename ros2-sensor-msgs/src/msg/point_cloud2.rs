use super::point_field::PointField;
use ros2_std_msgs::msg::Header;
use serde::Deserialize;

/// This message holds the description of one point entry in the
/// PointCloud2 message format.
///
/// uint8 INT8 = 1
/// uint8 UINT8 = 2
/// uint8 INT16 = 3
/// uint8 UINT16 = 4
/// uint8 INT32 = 5
/// uint8 UINT32 = 6
/// uint8 FLOAT32 = 7
/// uint8 FLOAT64 = 8
///
/// Common PointField names are x, y, z, intensity, rgb, rgba
///
/// string name # Name of field
///
/// uint32 offset # Offset from start of point struct
///
/// uint8 datatype # Datatype enumeration, see above
///
/// uint32 count # How many elements in the field
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct PointCloud2 {
    header: Header,
    height: u32,
    width: u32,
    fields: PointField,
    is_bigendian: bool,
    point_step: u32,
    row_step: u32,
    data: Vec<u8>,
    is_dense: bool,
}
