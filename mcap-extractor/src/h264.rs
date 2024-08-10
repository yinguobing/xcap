use indicatif::{ProgressBar, ProgressStyle};
use mcap::Message;
use openh264::{decoder::Decoder, formats::YUVSource, nal_units};
use ros2_sensor_msgs::msg::CompressedImage;
use std::{
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub struct Parser {
    // writer: File,
    writer: BufWriter<fs::File>,
    timestamps: Vec<u64>,
    frame_out: PathBuf,
    h264_out: PathBuf,
    h264_decoder: Decoder,
}

impl Parser {
    pub fn new(output_path: &PathBuf) -> Self {
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

    pub fn accumulate(&mut self, message: &Message) {
        self.timestamps.push(message.publish_time);
        let m: &[u8] = message.data.as_ref();
        let decompressed = zstd::stream::decode_all(m).unwrap();
        let img = cdr::deserialize_from::<_, CompressedImage, _>(
            decompressed.as_slice(),
            cdr::size::Infinite,
        )
        .unwrap();
        self.writer.write(&img.data).unwrap();
    }

    pub fn dump_frames(&mut self) {
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
            // On the first few frames this may fail, so you should check the result
            // a few packets before giving up.
            if let Ok(Some(yuv)) = self.h264_decoder.decode(packet) {
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
        }
        bar.finish();
    }
}
