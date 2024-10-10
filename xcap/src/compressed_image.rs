use crate::extractor::Extractor;
use log::error;
use mcap::Message;
use rerun::RecordingStream;
use ros2_sensor_msgs::msg::CompressedImage;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};

const ZSTD_MAGIC_NUMBER: [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interrupted.")]
    Interrupted,
    #[error("Init image from buf failed.")]
    ImageBuf,
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    CDR(#[from] cdr::Error),
    #[error("Image error. {0}")]
    Image(#[from] image::ImageError),
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
        let deserialized = cdr::deserialize_from::<_, CompressedImage, _>(
            serialized.as_slice(),
            cdr::size::Infinite,
        )
        .map_err(|e| Error::CDR(e))?;

        // Visualize?
        if let Some(rec) = &self.rec_stream {
            rec.set_time_seconds(
                "main",
                deserialized.header.stamp.sec as f64
                    + deserialized.header.stamp.nanosec as f64 * 1e-9,
            );
            rec.log(
                format!("image/{}", message.channel.topic.clone()),
                &rerun::EncodedImage::from_file_contents(deserialized.data.clone()),
            )?;
        }

        // Dump data?
        if self.dump_data {
            let path = self.output_dir.join(format!(
                "{}-{}.{}",
                deserialized.header.stamp.sec,
                deserialized.header.stamp.nanosec,
                deserialized.format
            ));
            std::fs::write(path, deserialized.data)?;
        }

        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
