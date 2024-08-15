use crate::extractor::Extractor;
use indicatif::{ProgressBar, ProgressStyle};
use mcap::Message;
use openh264::{decoder::Decoder, formats::YUVSource, nal_units};
use ros2_sensor_msgs::msg::CompressedImage;
use std::{
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interupted.")]
    Interupted,
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    CDR(#[from] cdr::Error),
}

pub struct Parser {
    // writer: File,
    writer: BufWriter<fs::File>,
    timestamps: Vec<u64>,
    frame_out: PathBuf,
    h264_out: PathBuf,
    h264_decoder: Decoder,
}

impl Parser {
    pub fn new(output_path: &Path) -> Self {
        // Create H264 file
        let h264_out = output_path.join("raw.h264");
        let file = fs::File::create(&h264_out).unwrap();
        let writer = BufWriter::new(file);

        // Create frame output dir
        let frame_out = output_path.join("frames");
        fs::create_dir_all(&frame_out).unwrap();

        // Video decoder
        let h264_decoder = Decoder::new().unwrap();

        Parser {
            writer,
            timestamps: vec![],
            frame_out,
            h264_out,
            h264_decoder,
        }
    }
}

impl Extractor for Parser {
    type Error = Error;
    fn step(&mut self, message: &Message) -> Result<(), Error> {
        self.timestamps.push(message.publish_time);
        let m: &[u8] = message.data.as_ref();
        let decompressed = zstd::stream::decode_all(m).map_err(|e| Error::Zstd(e))?;
        let img = cdr::deserialize_from::<_, CompressedImage, _>(
            decompressed.as_slice(),
            cdr::size::Infinite,
        )
        .map_err(|e| Error::CDR(e))?;
        self.writer.write_all(&img.data)?;
        Ok::<(), Error>(())
    }

    fn post_process(&mut self, sigint: Arc<AtomicBool>) -> Result<(), Error> {
        // Safety
        self.writer.flush().unwrap();

        // Create output directory
        fs::create_dir_all(&self.frame_out).unwrap();

        // Read in H.264 file
        let h264_in = fs::read(&self.h264_out).unwrap();

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
        for packet in nal_units(h264_in.as_slice()) {
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(Error::Interupted);
            }
            // On the first few frames this may fail, so you should check the result
            // a few packets before giving up.
            let decoded = self.h264_decoder.decode(packet);
            match decoded {
                Ok(Some(yuv)) => {
                    let rgb_buf_size = yuv.estimate_rgb_u8_size();
                    let mut buf: Vec<u8> = vec![0; rgb_buf_size];
                    yuv.write_rgb8(buf.as_mut_slice());
                    let (width, height) = yuv.dimensions();
                    let rgb = image::RgbImage::from_vec(width as u32, height as u32, buf).unwrap();
                    let img_path = self
                        .frame_out
                        .join(format!("{}.jpg", self.timestamps.get(idx).unwrap()));
                    rgb.save_with_format(img_path, image::ImageFormat::Jpeg)
                        .unwrap();
                    bar.inc(1);
                    idx += 1;
                }
                Ok(None) => continue,
                Err(e) => println!("DecodingError: {}", e),
            }
        }
        bar.finish();
        Ok(())
    }
}
