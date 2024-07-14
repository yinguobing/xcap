use clap::ValueEnum;

const H264_HEADER_960: &[u8; 32]= b"\x00\x00\x00\x01gd\x003K\n\xd0\x0c\x80K\xd0\x80\x00@\x00\x00\x0f\x00\x00B\x00\x00\x00\x01hJ\xe3\xcb";
const H264_HEADER_720: &[u8; 35] = b"\x00\x00\x00\x01gd\x003K\n\xd0\x0c\x80\xe7\xf3\xc2\x00\x00\x03\x00\x02\x00\x00\x03\x00y\x08\x00\x00\x00\x01hJ\xe3\xcb";

#[derive(Debug, Clone, ValueEnum)]
pub enum Model {
    DR1,
    DR2,
    PT1,
}

impl Model {
    pub fn h264_header(&self) -> &[u8] {
        match self {
            Self::DR1 | Self::DR2 => H264_HEADER_720,
            Self::PT1 => H264_HEADER_960,
        }
    }
}
