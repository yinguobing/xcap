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
    pub name: String,
    pub offset: u32,
    pub datatype: u8,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

impl PointField {
    pub fn decode_bytes(&self, data: &[u8]) -> Vec<Datatype> {
        let mut results: Vec<Datatype> = vec![];
        for _ in 0..self.count {
            let v = match self.datatype {
                1 => Datatype::INT8(i8::from_ne_bytes(data.try_into().unwrap())),
                2 => Datatype::UINT8(u8::from_ne_bytes(data.try_into().unwrap())),
                3 => Datatype::INT16(i16::from_ne_bytes(data.try_into().unwrap())),
                4 => Datatype::UINT16(u16::from_ne_bytes(data.try_into().unwrap())),
                5 => Datatype::INT32(i32::from_ne_bytes(data.try_into().unwrap())),
                6 => Datatype::UINT32(u32::from_ne_bytes(data.try_into().unwrap())),
                7 => Datatype::FLOAT32(f32::from_ne_bytes(data.try_into().unwrap())),
                8 => Datatype::FLOAT64(f64::from_ne_bytes(data.try_into().unwrap())),
                0_u8 | 9_u8..=u8::MAX => panic!("Invalid datatype."),
            };
            results.push(v);
        }
        results
    }

    pub fn size(&self) -> usize {
        match self.datatype {
            1 => std::mem::size_of::<i8>(),
            2 => std::mem::size_of::<u8>(),
            3 => std::mem::size_of::<i16>(),
            4 => std::mem::size_of::<u16>(),
            5 => std::mem::size_of::<i32>(),
            6 => std::mem::size_of::<u32>(),
            7 => std::mem::size_of::<f32>(),
            8 => std::mem::size_of::<f64>(),
            0_u8 | 9_u8..=u8::MAX => panic!("Can not get data size, invalid datatype."),
        }
    }
}
