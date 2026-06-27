#![allow(clippy::upper_case_acronyms)]
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
    CDR(#[from] cdr::Error),
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
        if let Some(path) = output_path {
            fs::create_dir_all(path).expect("Output directory should be created without error.");
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
                .map_err(Error::CDR)?;

        if let Some(rec) = &self.rec_stream {
            rec.set_time(
                "ros_publish",
                rerun::TimeCell::from_timestamp_nanos_since_epoch(message.publish_time as i64),
            );
            rec.set_time(
                "ros_log",
                rerun::TimeCell::from_timestamp_nanos_since_epoch(message.log_time as i64),
            );
            rec.set_time(
                "ros_idx",
                rerun::TimeCell::from_sequence(message.sequence as i64),
            );
            rec.set_timestamp_secs_since_epoch(
                "msg_header",
                image_msg.header.stamp.sec as f64 + image_msg.header.stamp.nanosec as f64 * 1e-9,
            );

            // Latency
            let ts_message_micros = i64::from(image_msg.header.stamp.sec) * 1_000_000
                + i64::from(image_msg.header.stamp.nanosec) / 1000;
            let ts_publish_micros = message.publish_time as i64 / 1000;
            rec.log(
                format!("latency/{}", &message.channel.topic.trim_start_matches("/")),
                &rerun::Scalars::single((ts_publish_micros - ts_message_micros) as f64 * 0.001),
            )?;

            if image_msg.encoding == "nv12" {
                rec.log(
                    format!("camera/{}", &message.channel.topic.trim_start_matches("/")),
                    &rerun::Image::from_pixel_format(
                        [image_msg.width, image_msg.height / 3 * 2],
                        rerun::PixelFormat::NV12,
                        image_msg.data.as_ref(),
                    ),
                )?;
            } else {
                rec.log(
                    format!("camera/{}", &message.channel.topic.trim_start_matches("/")),
                    &rerun::Image::from_rgb24(
                        image_msg.data.as_ref(),
                        [image_msg.width, image_msg.height],
                    ),
                )?;
            }
        }

        // Create output file
        if let Some(output_dir) = &self.output_dir {
            // Dump the binary data
            let mut file =
                fs::File::create(output_dir.join(format!("{}.bin", message.publish_time)))?;
            file.write_all(&image_msg.data)?;

            // Convert NV12 to RGB
            let img = if image_msg.encoding == "nv12" {
                let bi_planar_image = yuv::YuvBiPlanarImage {
                    y_plane: &image_msg.data[0..image_msg.data.len() / 3 * 2],
                    y_stride: image_msg.width,
                    uv_plane: &image_msg.data[image_msg.data.len() / 3 * 2..],
                    uv_stride: image_msg.width,
                    width: image_msg.width,
                    height: image_msg.height / 3 * 2,
                };
                let mut rgb = vec![0; image_msg.width as usize * image_msg.height as usize * 2];
                yuv::yuv_nv12_to_rgb(
                    &bi_planar_image,
                    &mut rgb,
                    image_msg.width * 3,
                    yuv::YuvRange::Full,
                    yuv::YuvStandardMatrix::Bt709,
                    yuv::YuvConversionMode::Balanced,
                )?;
                image::RgbImage::from_vec(image_msg.width, image_msg.height / 3 * 2, rgb)
                    .expect("RGB image convertion should not fail.")
            } else {
                image::RgbImage::from_vec(
                    image_msg.width,
                    image_msg.height,
                    image_msg.data.to_owned(),
                )
                .expect("RGB image convertion should not fail.")
            };
            img.save(output_dir.join(format!("{}.jpg", message.publish_time)))?;
        }
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }

    fn generates_3d_data(&self) -> bool {
        false
    }

    fn generates_2d_data(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creation_without_output() {
        let parser = Parser::new(&None, None);
        assert!(parser.generates_2d_data());
        assert!(!parser.generates_3d_data());
    }

    #[test]
    fn error_display() {
        let err = Error::CDR(cdr::Error::Message("test".to_string()));
        assert!(format!("{}", err).contains("CDR error"));
    }
}
