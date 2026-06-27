#![allow(clippy::upper_case_acronyms)]
use crate::extractor::Extractor;
use log::{error, info, warn};
use mcap::Message;
use openh264::{decoder::Decoder, formats::YUVSource, nal_units};
use rerun::RecordingStream;
use ros2_interfaces_humble::sensor_msgs::msg::CompressedImage;
use std::io::Read;
use std::{
    fs::{self, File},
    io::Write,
    iter::zip,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interrupted.")]
    Interrupted,
    #[error("Insufficient timestamp.")]
    Timestamp,
    #[error("Init image from buf failed.")]
    ImageBuf,
    #[error("OpenH264 error. {0}")]
    OH264(#[from] openh264::Error),
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    CDR(#[from] cdr::Error),
    #[error("Image error. {0}")]
    Image(#[from] image::ImageError),
}

type RgbImage = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub struct Parser {
    // Frame output directory
    frame_out: Option<PathBuf>,

    // Video decoder
    h264_cache: Vec<u8>,
    h264_decoder: Decoder,

    // Timestamp cache
    cache_timestamp: Vec<ros2_interfaces_humble::builtin_interfaces::msg::Time>,

    // Visualizer with rerun
    rec_stream: Option<RecordingStream>,

    // Should dump data to disk?
    file: Option<File>,

    // Should dump the raw binary data
    as_binary: bool,

    // The output image format
    output_image_format: image::ImageFormat,

    // Some counters to track the progress
    timestamp_count: usize,
    total_frame_count: usize,
}

impl Parser {
    pub fn new(
        output_path: &Option<PathBuf>,
        rerun_stream: Option<RecordingStream>,
        as_binary: bool,
        header: &Option<String>,
        output_image_format: &Option<String>,
    ) -> Self {
        // Create frame output directory
        let frame_out = output_path.as_ref().map(|p| p.join("frames"));
        if let Some(ref frame_dir) = frame_out {
            if !as_binary {
                fs::create_dir_all(frame_dir)
                    .expect("H264 frame output directory should be created");
            }
        }

        // Create H264 file
        let h264_out = output_path.as_ref().map(|p| p.join("raw.h264"));
        let file = if let Some(h_out) = h264_out.as_ref() {
            fs::create_dir_all(output_path.as_ref().unwrap())
                .expect("H264 output directory should be created");
            let _ = fs::File::create(h_out).expect("H264 output file should be created");
            Some(
                std::fs::OpenOptions::new()
                    .append(true)
                    .open(h_out)
                    .expect("Output file should be opened"),
            )
        } else {
            None
        };

        // Video decoder
        let mut h264_decoder = Decoder::new().unwrap();

        // H264 cache
        let mut h264_cache = vec![];
        if let Some(h264_header) = header {
            // read the whole file
            let mut f = std::fs::File::open(h264_header).expect("Header file should be found");
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)
                .expect("Header file should be readable");
            h264_cache.extend_from_slice(buffer.as_ref());
            for packet in nal_units(buffer.as_ref()) {
                let _ = h264_decoder.decode(packet);
            }
            if h264_out.is_some() {
                file.as_ref()
                    .unwrap()
                    .write_all(&buffer)
                    .expect("Video header should be written without error");
            }
        }

        // Try to match the output image format
        let output_image_format = match output_image_format {
            Some(f) => match f.to_lowercase().as_str() {
                "png" => image::ImageFormat::Png,
                "jpeg" => image::ImageFormat::Jpeg,
                "gif" => image::ImageFormat::Gif,
                "webp" => image::ImageFormat::WebP,
                "pnm" => image::ImageFormat::Pnm,
                "tiff" => image::ImageFormat::Tiff,
                "tga" => image::ImageFormat::Tga,
                "dds" => image::ImageFormat::Dds,
                "bmp" => image::ImageFormat::Bmp,
                "ico" => image::ImageFormat::Ico,
                "hdr" => image::ImageFormat::Hdr,
                "openexr" => image::ImageFormat::OpenExr,
                "farbfeld" => image::ImageFormat::Farbfeld,
                "avif" => image::ImageFormat::Avif,
                "qoi" => image::ImageFormat::Qoi,
                _ => {
                    log::error!("Image format not supported: {}. Using JPEG instead.", f);
                    image::ImageFormat::Jpeg
                }
            },
            _ => image::ImageFormat::Jpeg,
        };

        Parser {
            h264_cache,
            frame_out,
            h264_decoder,
            rec_stream: rerun_stream,
            cache_timestamp: vec![],
            timestamp_count: 0,
            total_frame_count: 0,
            file,
            as_binary,
            output_image_format,
        }
    }


    fn get_rgb(
        &mut self,
        stream: &[u8],
    ) -> Result<Vec<RgbImage>, Error> {
        let mut rgbs: Vec<RgbImage> = vec![];
        for packet in nal_units(stream) {
            // On the first few frames this may fail, so check the result a few more packets before giving up.
            let yuv = match self.h264_decoder.decode(packet)? {
                Some(v) => v,
                None => {
                    warn!("H264 decoder need more data");
                    continue;
                }
            };
            let rgb_buf_size = yuv.rgb8_len();
            let mut buf: Vec<u8> = vec![0; rgb_buf_size];
            yuv.write_rgb8(buf.as_mut_slice());
            let (width, height) = yuv.dimensions();
            rgbs.push(
                image::RgbImage::from_vec(width as u32, height as u32, buf)
                    .ok_or(Error::ImageBuf)?,
            );
        }
        Ok(rgbs)
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let serialized = self.decode(message)?;
        let raw = cdr::deserialize_from::<_, CompressedImage, _>(
            serialized.as_slice(),
            cdr::size::Infinite,
        )
        .map_err(Error::CDR)?;

        // Extend the cache
        if let Some(ref mut file) = &mut self.file {
            self.h264_cache.extend_from_slice(&raw.data);
            file.write_all(&raw.data)
                .expect("Video data should be written without error");
        }

        // Extract binary data only?
        if self.as_binary {
            return Ok(());
        }

        // Update timestamp
        let timestamp = raw.header.stamp.sec as f64 + raw.header.stamp.nanosec as f64 * 1e-9;
        self.cache_timestamp.push(raw.header.stamp.clone());
        self.timestamp_count += 1;

        // Get RGB frames. Frame may not always be available due to H264 decoding.
        // In this case, we cache the timestamp and try again later.
        let frames = match self.get_rgb(&raw.data) {
            Ok(frames) => frames,
            Err(e) => {
                error!("Failed to decode H264: {}", e);
                self.cache_timestamp.remove(0);
                return Ok(());
            }
        };
        if frames.is_empty() {
            self.cache_timestamp.remove(0);
            warn!("Can not get frame from current message: {:.9}", timestamp);
            return Ok(());
        }

        // Iterate the timestamps and frames.
        let frame_count = frames.len();
        self.total_frame_count += frame_count;
        for (rgb, ts) in zip(frames, self.cache_timestamp.clone()) {
            let timestamp = ts.sec as f64 + ts.nanosec as f64 * 1e-9;
            let timestamp_as_str = format!("{}", (timestamp * 1000.0) as u64);
            if let Some(frame_out) = &self.frame_out {
                let img_path = frame_out.join(format!(
                    "{}.{}",
                    timestamp_as_str,
                    self.output_image_format.extensions_str()[0]
                ));
                rgb.save_with_format(img_path, self.output_image_format)?;
            }
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
                rec.set_timestamp_secs_since_epoch("msg_header", timestamp);

                // Latency
                let ts_message_micros = i64::from(raw.header.stamp.sec) * 1_000_000
                    + i64::from(raw.header.stamp.nanosec) / 1000;
                let ts_publish_micros = message.publish_time as i64 / 1000;
                rec.log(
                    format!("latency/{}", &message.channel.topic.trim_start_matches("/")),
                    &rerun::Scalars::single((ts_publish_micros - ts_message_micros) as f64 * 0.001),
                )?;

                rec.log(
                    format!("camera/{}", message.channel.topic.trim_start_matches("/")),
                    &rerun::Image::from_rgb24(rgb.clone().into_raw(), [rgb.width(), rgb.height()]),
                )?;
            }
        }

        // Update the cache
        for _ in 0..frame_count {
            self.cache_timestamp.remove(0);
        }

        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        info!(
            "Extracted {} frames from {} messages",
            self.total_frame_count, self.timestamp_count
        );

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
    fn parser_creation_defaults() {
        let parser = Parser::new(&None, None, false, &None, &None);
        assert!(parser.generates_2d_data());
        assert!(!parser.generates_3d_data());
        assert!(!parser.as_binary);
        assert_eq!(parser.timestamp_count, 0);
        assert_eq!(parser.total_frame_count, 0);
    }

    #[test]
    fn parser_as_binary_mode() {
        let parser = Parser::new(&None, None, true, &None, &None);
        assert!(parser.as_binary);
        assert!(!parser.generates_3d_data());
    }

    #[test]
    fn image_format_valid_formats() {
        // Test each supported format
        let supported = [
            ("png", image::ImageFormat::Png),
            ("PNG", image::ImageFormat::Png),
            ("jpeg", image::ImageFormat::Jpeg),
            ("JPEG", image::ImageFormat::Jpeg),
            ("gif", image::ImageFormat::Gif),
            ("webp", image::ImageFormat::WebP),
            ("pnm", image::ImageFormat::Pnm),
            ("tiff", image::ImageFormat::Tiff),
            ("tga", image::ImageFormat::Tga),
            ("dds", image::ImageFormat::Dds),
            ("bmp", image::ImageFormat::Bmp),
            ("ico", image::ImageFormat::Ico),
            ("hdr", image::ImageFormat::Hdr),
            ("openexr", image::ImageFormat::OpenExr),
            ("farbfeld", image::ImageFormat::Farbfeld),
            ("avif", image::ImageFormat::Avif),
            ("qoi", image::ImageFormat::Qoi),
        ];
        for (name, expected_format) in &supported {
            let parser = Parser::new(&None, None, false, &None, &Some(name.to_string()));
            assert_eq!(
                parser.output_image_format.extensions_str(),
                expected_format.extensions_str(),
                "Format mismatch for: {}",
                name
            );
        }
    }

    #[test]
    fn image_format_unknown_falls_back_to_jpeg() {
        let parser = Parser::new(
            &None,
            None,
            false,
            &None,
            &Some("unknown_format".to_string()),
        );
        assert_eq!(
            parser.output_image_format.extensions_str(),
            image::ImageFormat::Jpeg.extensions_str()
        );
    }

    #[test]
    fn image_format_none_defaults_to_jpeg() {
        let parser = Parser::new(&None, None, false, &None, &None);
        assert_eq!(
            parser.output_image_format.extensions_str(),
            image::ImageFormat::Jpeg.extensions_str()
        );
    }

    #[test]
    fn error_display() {
        let err = Error::CDR(cdr::Error::Message("test".to_string()));
        assert!(format!("{}", err).contains("CDR error"));

        let io_err = std::io::Error::other("test");
        let err = Error::Zstd(io_err);
        assert!(format!("{}", err).contains("ZSTD error"));

        let err = Error::Interrupted;
        assert!(format!("{}", err).contains("Interrupted"));

        let err = Error::Timestamp;
        assert!(format!("{}", err).contains("Insufficient"));

        let err = Error::ImageBuf;
        assert!(format!("{}", err).contains("Init image from buf"));
    }
}
