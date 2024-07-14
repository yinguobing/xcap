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
        self.writer.write(&message.data).unwrap();
    }
}
