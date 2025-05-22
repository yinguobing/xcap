use crate::extractor::Extractor;
use crate::visual::Visual;
use log::error;
use mcap::Message;
use rerun::RecordingStream;
use ros2_sensor_msgs::msg::CompressedImage;
use std::{
    fs,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interrupted.")]
    Interrupted,
    #[error("Init image from buf failed.")]
    ImageBuf,
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    Cdr(#[from] cdr::Error),
    #[error("Image error. {0}")]
    Image(#[from] image::ImageError),
}

pub struct Parser {
    // Output directory
    output_dir: Option<PathBuf>,

    // Visualizer with rerun
    rec_stream: Option<RecordingStream>,
}

impl Parser {
    pub fn new(output_path: &Option<PathBuf>, rerun_stream: Option<RecordingStream>) -> Self {
        // Create output dir
        if let Some(output_dir) = output_path {
            fs::create_dir_all(output_dir).unwrap();
        }

        Parser {
            output_dir: output_path.clone(),
            rec_stream: rerun_stream,
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let serialized = self.decode(message)?;
        let deserialized = cdr::deserialize_from::<_, CompressedImage, _>(
            serialized.as_slice(),
            cdr::size::Infinite,
        )
        .map_err(Error::Cdr)?;

        // Visualization?
        if let Some(rec) = &self.rec_stream {
            deserialized.visualize(&message.channel.topic, rec)?
        }

        // Dump data?
        if let Some(output_dir) = &self.output_dir {
            let path = output_dir.join(format!(
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

impl Visual for CompressedImage {
    type VisualizeError = Box<dyn std::error::Error>;

    fn visualize(
        &self,
        entity_path: &str,
        rec: &RecordingStream,
    ) -> Result<(), Self::VisualizeError> {
        rec.set_timestamp_secs_since_epoch(
            "main",
            self.header.stamp.sec as f64 + self.header.stamp.nanosec as f64 * 1e-9,
        );
        rec.log(
            entity_path,
            &rerun::EncodedImage::from_file_contents(self.data.clone()),
        )?;
        Ok(())
    }
}
