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
    pub fn new(output_path: &PathBuf, vehicle_model: &Model, schema: Arc<mcap::Schema>) -> Self {
        // Create H264 file
        let h264_out = output_path.join("raw.h264");
        let file = File::create(&h264_out).unwrap();
        let writer = BufWriter::new(file);
        // let w = writer.write(vehicle_model.h264_header()).unwrap();
        // println!("w: {}", w);

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
        // println!("compressed  : {} {:x?}", m.len(), m);
        let decomrpessed = zstd::stream::decode_all(m).unwrap();
        // println!("decomrpessed: {} {:x?}", decomrpessed.len(), &decomrpessed);
        // println!("decomrpessed: {} {:?}", decomrpessed.len(), &decomrpessed);
        let img = cdr::deserialize_from::<_, Img, _>(decomrpessed.as_slice(), cdr::size::Infinite)
            .unwrap();
        // println!("{:?}", img);
        // println!("{:?}", img.data.len());
        // let offset: usize = *&decomrpessed[8] as usize + 4;
        // println!("offset   : {:?}", offset);
        // // let dynamic_msg = message_definitions[3].as_ref().unwrap();
        // // let decoded_msg = dynamic_msg.decode(&decomrpessed[..]).unwrap();
        // let raw = &decomrpessed[offset..];
        self.writer.write(&img.data).unwrap();
    }
}
