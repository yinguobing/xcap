use crate::extractor::Extractor;
use indicatif::{ProgressBar, ProgressStyle};
use mcap::Message;
use openh264::{decoder::Decoder, formats::YUVSource, nal_units};
use ros2_sensor_msgs::msg::CompressedImage;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interupted.")]
    Interupted,
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

pub struct Parser {
    timestamps: Vec<u64>,
    frame_out: PathBuf,
    h264_cache: Vec<u8>,
    h264_out: PathBuf,
    h264_decoder: Decoder,
}

impl Parser {
    pub fn new(output_path: &Path) -> Self {
        // Create H264 file
        let h264_out = output_path.join("raw.h264");

        // Create frame output dir
        let frame_out = output_path.join("frames");
        fs::create_dir_all(&frame_out).unwrap();

        // Video decoder
        let h264_decoder = Decoder::new().unwrap();

        Parser {
            h264_cache: vec![],
            timestamps: vec![],
            frame_out,
            h264_out,
            h264_decoder,
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        self.timestamps.push(message.publish_time);
        let m: &[u8] = message.data.as_ref();
        let decompressed = zstd::stream::decode_all(m).map_err(|e| Error::Zstd(e))?;
        let raw = cdr::deserialize_from::<_, CompressedImage, _>(
            decompressed.as_slice(),
            cdr::size::Infinite,
        )
        .map_err(|e| Error::CDR(e))?;
        self.h264_cache.extend_from_slice(&raw.data);
        Ok(())
    }

    fn post_process(&mut self, sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        // Create output directory for frames
        fs::create_dir_all(&self.frame_out)?;

        // Create H264 file
        let mut file = fs::File::create(&self.h264_out)?;
        file.write_all(&self.h264_cache)?;

        // Split H.264 into NAL units and decode each.
        let bar = ProgressBar::new(self.timestamps.len() as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        let mut idx: usize = 0;
        for packet in nal_units(&self.h264_cache.as_slice()) {
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(Box::new(Error::Interupted));
            }
            // On the first few frames this may fail, so you should check the result
            // a few packets before giving up.
            let Some(yuv) = self.h264_decoder.decode(packet)? else {
                continue;
            };

            let rgb_buf_size = yuv.estimate_rgb_u8_size();
            let mut buf: Vec<u8> = vec![0; rgb_buf_size];
            yuv.write_rgb8(buf.as_mut_slice());
            let (width, height) = yuv.dimensions();
            let rgb = image::RgbImage::from_vec(width as u32, height as u32, buf)
                .ok_or(Error::ImageBuf)?;
            let img_path = self.frame_out.join(format!(
                "{}.jpg",
                self.timestamps.get(idx).ok_or(Error::Timestamp)?
            ));
            rgb.save_with_format(img_path, image::ImageFormat::Jpeg)?;
            bar.inc(1);
            idx += 1;
        }
        bar.finish();
        Ok(())
    }
}
