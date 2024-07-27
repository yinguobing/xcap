use crate::vehicle::Model;
use mcap::Message;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct Parser {
    writer: BufWriter<File>,
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
        let mut decomrpessed = zstd::stream::decode_all(m).unwrap();
        // println!("decomrpessed: {} {:x?}", decomrpessed.len(), &decomrpessed);
        // println!("decomrpessed: {} {:?}", decomrpessed.len(), &decomrpessed);
        let offset: usize = *&decomrpessed[8] as usize + 4;
        println!("offset   : {:?}", offset);
        // let dynamic_msg = message_definitions[3].as_ref().unwrap();
        // let decoded_msg = dynamic_msg.decode(&decomrpessed[..]).unwrap();
        let raw = &decomrpessed[offset..];
        self.writer.write(raw).unwrap();
    }
}
