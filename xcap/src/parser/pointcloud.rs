use crate::extractor::Extractor;
use colorgrad::Gradient;
use mcap::Message;
use pcd_rs::{DataKind, DynRecord, DynWriter, Field, Schema, ValueKind, WriterInit};
use rerun::RecordingStream;
use ros2_interfaces_humble::sensor_msgs::msg::{PointCloud2, PointField};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    iter::zip,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    Cdr(#[from] cdr::Error),
}

pub struct Parser {
    // Output directory
    output_dir: Option<PathBuf>,

    // Visualizer with rerun
    rec_stream: Option<RecordingStream>,

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
        output_path: &Option<PathBuf>,
        rerun_stream: Option<RecordingStream>,
        spatial_scale: &Option<f32>,
        intensity_scale: &Option<f32>,
    ) -> Self {
        // Create output dir
        if let Some(output_dir) = output_path {
            fs::create_dir_all(output_dir).unwrap();
        }

        Parser {
            output_dir: output_path.clone(),
            rec_stream: rerun_stream,
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

    fn decode_pointcloud(&self, message: &PointCloud2) -> HashMap<String, Vec<f64>> {
        let mut decoded: HashMap<String, Vec<f64>> = HashMap::new();
        for field in message.fields.iter() {
            let field_name = field.name.as_str();
            let field_size = match field.datatype {
                PointField::INT8 => std::mem::size_of::<i8>(),
                PointField::UINT8 => std::mem::size_of::<u8>(),
                PointField::INT16 => std::mem::size_of::<i16>(),
                PointField::UINT16 => std::mem::size_of::<u16>(),
                PointField::INT32 => std::mem::size_of::<i32>(),
                PointField::UINT32 => std::mem::size_of::<u32>(),
                PointField::FLOAT32 => std::mem::size_of::<f32>(),
                PointField::FLOAT64 => std::mem::size_of::<f64>(),
                0_u8 | 9_u8..=u8::MAX => panic!("Can not get data size, invalid datatype."),
            };
            let decode_fun: fn(&[u8]) -> f64 = match field.datatype {
                PointField::INT8 => |x| f64::from(i8::from_ne_bytes(x.try_into().unwrap())),
                PointField::UINT8 => |x| f64::from(u8::from_ne_bytes(x.try_into().unwrap())),
                PointField::INT16 => |x| f64::from(i16::from_ne_bytes(x.try_into().unwrap())),
                PointField::UINT16 => |x| f64::from(u16::from_ne_bytes(x.try_into().unwrap())),
                PointField::INT32 => |x| f64::from(i32::from_ne_bytes(x.try_into().unwrap())),
                PointField::UINT32 => |x| f64::from(u32::from_ne_bytes(x.try_into().unwrap())),
                PointField::FLOAT32 => |x| f64::from(f32::from_ne_bytes(x.try_into().unwrap())),
                PointField::FLOAT64 => |x| f64::from_ne_bytes(x.try_into().unwrap()),
                0_u8 | 9_u8..=u8::MAX => panic!("Can not match decode function, invalid datatype."),
            };
            let num_points = message.width * message.height;
            let mut values: Vec<f64> = Vec::with_capacity(num_points as usize);
            for idx in 0..num_points {
                let idx_start = message.point_step * idx + field.offset;
                let idx_end = idx_start + field.count * field_size as u32;
                let buf = &message.data[idx_start as usize..idx_end as usize];
                values.push(decode_fun(buf));
            }
            decoded.insert(field_name.to_string(), values);
        }
        decoded
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let serialized = self.decode(message)?;
        let cloud_msg =
            cdr::deserialize_from::<_, PointCloud2, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(Error::Cdr)?;

        // Extract points and intensity
        let decoded = self.decode_pointcloud(&cloud_msg);
        let points = decoded["x"]
            .iter()
            .zip(decoded["y"].iter())
            .zip(decoded["z"].iter())
            .map(|((x, y), z)| {
                rerun::Position3D::new(
                    *x as f32 * self.spatial_scale,
                    *y as f32 * self.spatial_scale,
                    *z as f32 * self.spatial_scale,
                )
            });
        let intensity = decoded["intensity"]
            .iter()
            .map(|p| *p as f32 * self.intensity_scale);

        // Visualize?
        if let Some(rec) = &self.rec_stream {
            let colors = intensity.clone().map(|i| {
                let [r, g, b, a] = self.color_map.at(i).to_rgba8();
                rerun::Color::from_unmultiplied_rgba(r, g, b, a)
            });
            rec.set_timestamp_secs_since_epoch(
                "main",
                cloud_msg.header.stamp.sec as f64 + cloud_msg.header.stamp.nanosec as f64 * 1e-9,
            );
            rec.log(
                message.channel.topic.clone(),
                &rerun::Points3D::new(points.clone())
                    .with_colors(colors)
                    .with_radii([0.01]),
            )?;
        }

        // Create output file
        if let Some(output_dir) = &self.output_dir {
            // Output binary file
            let mut output_file = output_dir.join(format!("{}.bin", message.publish_time));
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
