use crate::extractor::Extractor;
use colorgrad::Gradient;
use mcap::Message;
use pcd_rs::{DataKind, DynRecord, DynWriter, Field, Schema, ValueKind, WriterInit};
use rerun::RecordingStream;
use ros2_sensor_msgs::msg::{PointCloud2, PointCloud2Iterator};
use std::{
    fs,
    io::Write,
    iter::zip,
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
        let cloud_msg =
            cdr::deserialize_from::<_, PointCloud2, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(|e| Error::CDR(e))?;

        // Extract points and intensity
        let points: Vec<rerun::Position3D> = PointCloud2Iterator::new(&cloud_msg)
            .into_iter()
            .map(|p| {
                rerun::Position3D::new(
                    f32::from(p[0][0]) * self.spatial_scale,
                    f32::from(p[1][0]) * self.spatial_scale,
                    f32::from(p[2][0]) * self.spatial_scale,
                )
            })
            .collect();
        let intensity: Vec<f32> = PointCloud2Iterator::new(&cloud_msg)
            .into_iter()
            .map(|p| f32::from(p[3][0]) * self.intensity_scale)
            .collect();

        // Visualize?
        if let Some(rec) = &self.rec_stream {
            let colors = intensity.iter().map(|i| {
                let [r, g, b, a] = self.color_map.at(*i).to_rgba8();
                rerun::Color::from_unmultiplied_rgba(r, g, b, a)
            });
            rec.set_time_seconds(
                "main",
                cloud_msg.header.stamp.sec as f64 + cloud_msg.header.stamp.nanosec as f64 * 1e-9,
            );
            rec.log(
                format!("/world/cloud/{}", message.channel.topic.clone()),
                &rerun::Points3D::new(points.clone())
                    .with_colors(colors)
                    .with_radii([0.01]),
            )?;
        }

        // Create output file
        if self.dump_data {
            // Output binary file
            let mut output_file = self
                .output_dir
                .join(format!("{}.bin", message.publish_time));
            let mut file = fs::File::create(&output_file)?;
            file.write_all(&cloud_msg.data)?;

            // Output PCD file
            let schema = vec![
                ("x", ValueKind::F32, 1),
                ("y", ValueKind::F32, 1),
                ("z", ValueKind::F32, 1),
                ("intensity", ValueKind::F32, 1),
            ];

            // Build a writer
            output_file.set_extension("pcd");
            let mut writer: DynWriter<_> = WriterInit {
                width: cloud_msg.width as u64,
                height: cloud_msg.height as u64,
                viewpoint: Default::default(),
                data_kind: DataKind::Ascii,
                schema: Some(Schema::from_iter(schema)),
            }
            .create(&output_file)?;

            // Send the points to the writer
            for (point, itn) in zip(points, intensity) {
                let r: DynRecord = DynRecord(vec![
                    Field::F32(vec![point[0]]),
                    Field::F32(vec![point[1]]),
                    Field::F32(vec![point[2]]),
                    Field::F32(vec![itn]),
                ]);
                writer.push(&r)?;
            }

            // Finalize the writer
            writer.finish()?;
        }

        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
