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
    rec: RecordingStream,
}

impl Parser {
    pub fn new(output_path: &Path) -> Self {
        // Create output dir
        fs::create_dir_all(output_path).unwrap();

        // Visualizer
        let rec = rerun::RecordingStreamBuilder::new("rerun_example_minimal")
            .spawn()
            .unwrap();

        Parser {
            output_dir: output_path.into(),
            rec,
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
        let points_for_vis = PointCloud2Iterator::new(&points)
            .into_iter()
            .map(|p| glam::vec3((p[0][0]).into(), p[1][0].into(), p[2][0].into()));

        let colors = PointCloud2Iterator::new(&points)
            .into_iter()
            .map(|_| rerun::Color::from_rgb(255, 255, 255));

        self.rec.log(
            "cloud",
            &rerun::Points3D::new(points_for_vis)
                .with_colors(colors)
                .with_radii([1.0]),
        )?;

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
