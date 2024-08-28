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
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct PointField {
    name: String,
    offset: u32,
    datatype: u8,
    count: u32,
}

pub enum Datatype {
    INT8(i8),
    UINT8(u8),
    INT16(i16),
    UINT16(u16),
    INT32(i32),
    UINT32(u32),
    FLOAT32(f32),
    FLOAT64(f64),
}
