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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    CDR(#[from] cdr::Error),
}

pub struct Parser {
    output_dir: PathBuf,

    // Visualizer with rerun
    rec_stream: Option<Arc<RecordingStream>>,
}

impl Parser {
    pub fn new(output_path: &Path, rerun_stream: Option<Arc<RecordingStream>>) -> Self {
        // Create output dir
        fs::create_dir_all(output_path).unwrap();

        Parser {
            output_dir: output_path.into(),
            rec_stream: rerun_stream,
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let m: &[u8] = message.data.as_ref();
        let decompressed = zstd::stream::decode_all(m).map_err(|e| Error::Zstd(e))?;
        let points = cdr::deserialize_from::<_, PointCloud2, _>(
            decompressed.as_slice(),
            cdr::size::Infinite,
        )
        .map_err(|e| Error::CDR(e))?;

        if let Some(rec) = &self.rec_stream {
            let points_for_vis = PointCloud2Iterator::new(&points)
                .into_iter()
                .map(|p| glam::vec3((p[0][0]).into(), p[1][0].into(), p[2][0].into()));

            let colors = PointCloud2Iterator::new(&points)
                .into_iter()
                .map(|_| rerun::Color::from_rgb(255, 255, 255));

            rec.log(
                "cloud",
                &rerun::Points3D::new(points_for_vis)
                    .with_colors(colors)
                    .with_radii([1.0]),
            )?;
            rec.log(
                "objects",
                &rerun::Boxes3D::from_centers_and_half_sizes(
                    [(0.0, 0.0, 0.0)],
                    [(300.0, 200.0, 100.0)],
                )
                .with_quaternions([
                    rerun::Quaternion::from_xyzw([0.0, 0.0, 0.382683, 0.923880]), // 45 degrees around Z
                ])
                .with_radii([0.05])
                .with_colors([rerun::Color::from_rgb(0, 255, 0)])
                .with_fill_mode(rerun::FillMode::DenseWireframe),
            )?;
        }

        // Create output file
        let mut file = fs::File::create(
            &self
                .output_dir
                .join(format!("{}.bin", message.publish_time)),
        )?;
        file.write_all(&points.data)?;
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
