use crate::extractor::Extractor;
use mcap::Message;
use rerun::{external::glam, RecordingStream};
use ros2_sensor_msgs::msg::{PointCloud2, PointCloud2Iterator};
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

    // Scale the points? This could be usefull if users want to visualize the pointcloud in a
    // different scale.
    scale: f32,
}

impl Parser {
    pub fn new(
        output_path: &Path,
        rerun_stream: Option<RecordingStream>,
        dump_data: bool,
        scale: Option<f32>,
    ) -> Self {
        // Create output dir
        if dump_data {
            fs::create_dir_all(output_path).unwrap();
        }

        Parser {
            output_dir: output_path.into(),
            rec_stream: rerun_stream,
            dump_data,
            scale: scale.unwrap_or(1.0),
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
        let points =
            cdr::deserialize_from::<_, PointCloud2, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(|e| Error::CDR(e))?;

        if let Some(rec) = &self.rec_stream {
            let points_for_vis = PointCloud2Iterator::new(&points).into_iter().map(|p| {
                let v = glam::vec3((p[0][0]).into(), p[1][0].into(), p[2][0].into());
                v * self.scale
            });

            let colors = PointCloud2Iterator::new(&points)
                .into_iter()
                .map(|_| rerun::Color::from_rgb(255, 255, 255));
            rec.set_time_seconds(
                "main",
                points.header.stamp.sec as f64 + points.header.stamp.nanosec as f64 * 1e-9,
            );
            rec.log(
                format!("cloud/{}", message.channel.topic.clone()),
                &rerun::Points3D::new(points_for_vis)
                    .with_colors(colors)
                    .with_radii([0.01]),
            )?;
        }

        // Create output file
        if self.dump_data {
            let mut file = fs::File::create(
                &self
                    .output_dir
                    .join(format!("{}.bin", message.publish_time)),
            )?;
            file.write_all(&points.data)?;
        }
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
