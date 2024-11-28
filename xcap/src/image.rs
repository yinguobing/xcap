use crate::extractor::Extractor;
use mcap::Message;
use rerun::RecordingStream;
use ros2_sensor_msgs::msg::Image;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};

const ZSTD_MAGIC_NUMBER: [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    CDR(#[from] cdr::Error),
}

pub struct Parser {
    // Output directory
    output_dir: PathBuf,

    // Visualizer with rerun
    rec_stream: Option<RecordingStream>,

    // Should dump data to disk
    dump_data: bool,
}

impl Parser {
    pub fn new(output_path: &Path, rerun_stream: Option<RecordingStream>, dump_data: bool) -> Self {
        // Create output dir
        if dump_data {
            fs::create_dir_all(output_path).unwrap();
        }

        Parser {
            output_dir: output_path.into(),
            rec_stream: rerun_stream,
            dump_data,
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let buf = message.data.as_ref();
        let serialized = if &message.data[..4] == ZSTD_MAGIC_NUMBER {
            zstd::stream::decode_all(buf).map_err(|e| Error::Zstd(e))?
        } else {
            message.data.to_vec()
        };
        let image_msg =
            cdr::deserialize_from::<_, Image, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(|e| Error::CDR(e))?;

        if let Some(rec) = &self.rec_stream {
            rec.set_time_seconds(
                "main",
                image_msg.header.stamp.sec as f64 + image_msg.header.stamp.nanosec as f64 * 1e-9,
            );
            // rec.log(
            //     format!("image/{}", message.channel.topic.clone()),
            //     &rerun::Image::new(image_msg.data),
            // )?;
        }

        // Create output file
        if self.dump_data {
            let mut file = fs::File::create(
                &self
                    .output_dir
                    .join(format!("{}.bin", message.publish_time)),
            )?;
            file.write_all(&image_msg.data)?;
        }
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
