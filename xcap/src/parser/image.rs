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
        let serialized = self.decode(message)?;
        let image_msg =
            cdr::deserialize_from::<_, Image, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(|e| Error::CDR(e))?;

        if image_msg.encoding != "nv12" {
            return Ok(());
        }

        let height_rgb = (image_msg.height as f32 / 1.5) as usize;
        let mut rgb = vec![0u8; height_rgb * image_msg.width as usize * 3];
        let _ret = unsafe {
            libyuv::nv12_to_raw(
                image_msg.data.as_ptr(),
                image_msg.width as i32,
                image_msg
                    .data
                    .as_ptr()
                    .add(image_msg.width as usize * height_rgb),
                image_msg.width as i32,
                rgb.as_mut_ptr(),
                image_msg.width as i32 * 3,
                image_msg.width as i32,
                height_rgb as i32,
            )
        };
        if let Some(rec) = &self.rec_stream {
            rec.set_timestamp_secs_since_epoch(
                "main",
                image_msg.header.stamp.sec as f64 + image_msg.header.stamp.nanosec as f64 * 1e-9,
            );
            rec.log(
                format!("image/{}", message.channel.topic.clone()),
                &rerun::Image::from_elements(
                    rgb.as_ref(),
                    [image_msg.width, height_rgb as u32],
                    rerun::ColorModel::RGB,
                ),
            )?;
        }

        // Create output file
        if self.dump_data {
            let mut file = fs::File::create(
                &self
                    .output_dir
                    .join(format!("{}.bin", message.publish_time)),
            )?;
            file.write_all(&image_msg.data)?;
            let img = image::RgbImage::from_vec(image_msg.width, height_rgb as u32, rgb)
                .expect("Image should be valid");

            img.save(
                &self
                    .output_dir
                    .join(format!("{}.jpg", message.publish_time)),
            )?;
        }
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
