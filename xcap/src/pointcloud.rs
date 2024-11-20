use crate::extractor::Extractor;
use colorgrad::Gradient;
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

    // Scale the points in spatial domain? This could be usefull if users want to visualize the pointcloud in a
    // different spatial scale.
    spatial_scale: f32,

    // Intensity scale. This is used to scale the intensity values to a range [0, 1].
    intensity_scale: f32,

    // Color map. Map point cloud intensity to a color.
    color_map: colorgrad::LinearGradient,
}

impl Parser {
    pub fn new(
        output_path: &Path,
        rerun_stream: Option<RecordingStream>,
        dump_data: bool,
        spatial_scale: Option<f32>,
        intensity_scale: Option<f32>,
    ) -> Self {
        // Create output dir
        if dump_data {
            fs::create_dir_all(output_path).unwrap();
        }

        Parser {
            output_dir: output_path.into(),
            rec_stream: rerun_stream,
            dump_data,
            spatial_scale: spatial_scale.unwrap_or(1.0),
            intensity_scale: intensity_scale.unwrap_or(1.0),
            color_map: colorgrad::GradientBuilder::new()
                .html_colors(&["#00F", "#FFF", "gold"])
                .domain(&[0.0, 0.3, 0.6])
                .mode(colorgrad::BlendMode::LinearRgb)
                .build::<colorgrad::LinearGradient>()
                .expect("Color map should be created"),
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
                v * self.spatial_scale
            });
            let intensity = PointCloud2Iterator::new(&points).into_iter().map(|p| {
                f32::from(p.last().unwrap().last().unwrap().clone()) * self.intensity_scale
            });
            let colors = intensity.map(|i| {
                let [r, g, b, a] = self.color_map.at(i).to_rgba8();
                rerun::Color::from_unmultiplied_rgba(r, g, b, a)
            });

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
