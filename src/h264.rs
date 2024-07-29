use crate::vehicle::Model;
use mcap::Message;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct Parser {
    writer: BufWriter<File>,
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
    pub fn new(output_path: PathBuf, vehicle_model: Model) -> Self {
        let h264_out = output_path.join("raw.h264");
        let file = File::create(h264_out).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(vehicle_model.h264_header()).unwrap();
        Parser { writer }
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
