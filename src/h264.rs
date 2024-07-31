use crate::vehicle::Model;
use mcap::Message;
use openh264::decoder::Decoder;
use openh264::formats::YUVSource;
use openh264::nal_units;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Arc;

pub struct Parser {
    // writer: File,
    writer: BufWriter<File>,
    timestamps: Vec<u32>,
    frame_out: PathBuf,
    h264_out: PathBuf,
    h264_decoder: Decoder,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Time {
    sec: i32,
    nanosec: u32,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Header {
    stamp: Time,
    frame_id: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Img {
    header: Header,
    format: String,
    data: Vec<u8>,
}

impl Parser {
    pub fn new(output_path: &PathBuf) -> Self {
        // Create H264 file
        let h264_out = output_path.join("raw.h264");
        let file = File::create(&h264_out).unwrap();
        let writer = BufWriter::new(file);

        // Create frame output dir
        let frame_out = output_path.join("frames");
        std::fs::create_dir_all(&frame_out).unwrap();

        // Video decoder
        let h264_decoder = Decoder::new().unwrap();

        Parser {
            writer,
            timestamps: vec![],
            frame_out,
            h264_out,
            h264_decoder,
        }
    }

    pub fn process(&mut self, message: &Message) {
        let m: &[u8] = message.data.as_ref();
        let decomrpessed = zstd::stream::decode_all(m).unwrap();
        let img = cdr::deserialize_from::<_, Img, _>(decomrpessed.as_slice(), cdr::size::Infinite)
            .unwrap();
        self.writer.write(&img.data).unwrap();
    }
}
