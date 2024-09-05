use super::point_field::{Datatype, PointField};
use ros2_std_msgs::msg::Header;
use serde::Deserialize;

/// This message holds a collection of N-dimensional points, which may
/// contain additional information such as normals, intensity, etc. The
/// point data is stored as a binary blob, its layout described by the
/// contents of the "fields" array.
///
/// The point cloud data may be organized 2d (image-like) or 1d (unordered).
///
/// Point clouds organized as 2d images may be produced by camera depth sensors
/// such as stereo or time-of-flight.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct PointCloud2 {
    /// Time of sensor data acquisition, and the coordinate frame ID (for 3d points).
    pub header: Header,

    /// 2D structure of the point cloud. If the cloud is unordered, height is
    /// 1 and width is the length of the point cloud.
    pub height: u32,
    pub width: u32,

    /// Describes the channels and their layout in the binary data blob.
    pub fields: Vec<PointField>,

    /// Is this data bigendian?
    pub is_bigendian: u8,

    /// Length of a point in bytes
    pub point_step: u32,

    /// Length of a row in bytes
    pub row_step: u32,

    /// Actual point data, size is (row_step*height)
    pub data: Vec<u8>,

    /// True if there are no invalid points
    pub is_dense: u8,
}

impl PointCloud2 {
    pub fn len(&self) -> usize {
        self.width as usize * self.height as usize
    }
    pub fn decode_by_idx(&self, idx: usize) -> Option<Vec<Vec<Datatype>>> {
        if idx >= self.len() {
            return None;
        }
        let mut results: Vec<Vec<Datatype>> = Vec::new();
        let buf_point =
            &self.data[self.point_step as usize * idx..self.point_step as usize * (idx + 1)];
        for field in self.fields.iter() {
            let data = &buf_point[field.offset as usize
                ..(field.offset + field.count * field.size() as u32) as usize];
            results.push(field.decode_bytes(data));
        }
        Some(results)
    }
}
