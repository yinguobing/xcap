use crate::extractor::Extractor;
use mcap::Message;
use rerun::RecordingStream;
use ros2_interfaces_humble::sensor_msgs::msg::Image;
use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    Cdr(#[from] cdr::Error),
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
        let image_msg =
            cdr::deserialize_from::<_, Image, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(Error::Cdr)?;

        let mut buf: Vec<u8> = vec![];
        let (height_rgb, rgb) = if image_msg.encoding != "nv12" {
            (image_msg.height, &image_msg.data)
        } else {
            let height_rgb = (image_msg.height as f32 / 1.5) as usize;
            buf.resize(image_msg.width as usize * height_rgb * 3, 0);
            let _ret = unsafe {
                libyuv::nv12_to_raw(
                    image_msg.data.as_ptr(),
                    image_msg.width as i32,
                    image_msg
                        .data
                        .as_ptr()
                        .add(image_msg.width as usize * height_rgb),
                    image_msg.width as i32,
                    buf.as_mut_ptr(),
                    image_msg.width as i32 * 3,
                    image_msg.width as i32,
                    height_rgb as i32,
                )
            };
            (height_rgb as u32, &buf)
        };

        // Visualization
        if let Some(rec) = &self.rec_stream {
            rec.set_timestamp_secs_since_epoch(
                "main",
                image_msg.header.stamp.sec as f64 + image_msg.header.stamp.nanosec as f64 * 1e-9,
            );
            rec.log(
                message.channel.topic.clone(),
                &rerun::Image::from_elements(
                    rgb,
                    [image_msg.width, height_rgb],
                    rerun::ColorModel::RGB,
                ),
            )?;
        }

        // Dump data
        if let Some(output_dir) = &self.output_dir {
            let mut file =
                fs::File::create(output_dir.join(format!("{}.bin", message.publish_time)))?;
            file.write_all(&image_msg.data)?;
            let img = image::RgbImage::from_vec(image_msg.width, height_rgb, rgb.to_owned())
                .expect("Image should be valid");

            img.save(output_dir.join(format!("{}.jpg", message.publish_time)))?;
        }
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
