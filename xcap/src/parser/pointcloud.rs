#![allow(clippy::upper_case_acronyms)]
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
    CDR(#[from] cdr::Error),
}

pub struct Parser<'a> {
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

    // Should dump the raw binary data
    as_binary: &'a bool,

    // PCD schema
    pcd_schema: Option<Schema>,

    // Output file suffix
    suffix: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(
        output_path: &Option<PathBuf>,
        rerun_stream: &Option<RecordingStream>,
        spatial_scale: &Option<f32>,
        intensity_scale: &Option<f32>,
        as_binary: &'a bool,
    ) -> Self {
        // Create output dir
        if let Some(output_path) = output_path {
            fs::create_dir_all(output_path).unwrap();
        }

        let pcd_schema = if *as_binary {
            None
        } else {
            Some(Schema::from_iter(vec![
                ("x", ValueKind::F32, 1),
                ("y", ValueKind::F32, 1),
                ("z", ValueKind::F32, 1),
                ("intensity", ValueKind::F32, 1),
            ]))
        };
        let suffix = if *as_binary { "bin" } else { "pcd" };

        Parser {
            output_dir: output_path.clone(),
            rec_stream: rerun_stream.clone(),
            spatial_scale: spatial_scale.unwrap_or(1.0),
            intensity_scale: intensity_scale.unwrap_or(1.0),
            color_map: colorgrad::GradientBuilder::new()
                .html_colors(&["#00F", "#FFF", "gold"])
                .domain(&[0.0, 0.3, 0.6])
                .mode(colorgrad::BlendMode::LinearRgb)
                .build::<colorgrad::LinearGradient>()
                .expect("Color map should be created"),
            as_binary,
            pcd_schema,
            suffix,
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

impl<'a> Extractor for Parser<'a> {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let serialized = self.decode(message)?;
        let cloud_msg =
            cdr::deserialize_from::<_, PointCloud2, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(Error::CDR)?;

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

        // Timestamp
        let timestamp =
            cloud_msg.header.stamp.sec as f64 + cloud_msg.header.stamp.nanosec as f64 * 1e-9;

        // Visualize?
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
            let ts_message_micros = i64::from(cloud_msg.header.stamp.sec) * 1_000_000
                + i64::from(cloud_msg.header.stamp.nanosec) / 1000;
            let ts_publish_micros = message.publish_time as i64 / 1000;
            rec.log(
                format!("latency/{}", &message.channel.topic.trim_start_matches("/")),
                &rerun::Scalars::single((ts_publish_micros - ts_message_micros) as f64 * 0.001),
            )?;

            let colors = intensity.clone().map(|i| {
                let [r, g, b, a] = self.color_map.at(i).to_rgba8();
                rerun::Color::from_unmultiplied_rgba(r, g, b, a)
            });
            let mut point_groups: HashMap<i16, (Vec<rerun::Position3D>, Vec<rerun::Color>)> =
                HashMap::new();
            let default_sensor_id = vec![0.0; points.len()];
            let sensor_idicies = decoded.get("index").unwrap_or(&default_sensor_id);
            for ((p, i), c) in points.clone().zip(sensor_idicies).zip(colors) {
                point_groups
                    .entry(*i as i16)
                    .and_modify(|v| {
                        v.0.push(p);
                        v.1.push(c)
                    })
                    .or_insert((vec![], vec![]));
            }
            for (sensor_idx, (ps, cs)) in point_groups {
                rec.log(
                    format!(
                        "world/ego/{}/{}",
                        &message.channel.topic.trim_start_matches("/"),
                        sensor_idx
                    ),
                    &rerun::Points3D::new(ps).with_colors(cs).with_radii([0.01]),
                )?;
            }
        }

        // Create output file
        if let Some(output_dir) = &self.output_dir {
            // Output file
            let timestamp_as_str = format!("{}", (timestamp * 1000.0) as u64);
            let output_path = output_dir.join(format!("{}.{}", timestamp_as_str, self.suffix));

            // binary file?
            if *self.as_binary {
                let mut file = fs::File::create(&output_path)?;
                file.write_all(&cloud_msg.data)?;
            } else {
                // PCD file
                let mut writer: DynWriter<_> = WriterInit {
                    width: cloud_msg.width as u64,
                    height: cloud_msg.height as u64,
                    viewpoint: Default::default(),
                    data_kind: DataKind::Ascii,
                    schema: self.pcd_schema.clone(),
                }
                .create(&output_path)?;

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
        }

        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }

    fn generates_2d_data(&self) -> bool {
        false
    }

    fn generates_3d_data(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creation() {
        let parser = Parser::new(&None, &None, &None, &None, &false);
        assert!(!parser.generates_2d_data());
        assert!(parser.generates_3d_data());
    }

    #[test]
    fn error_display() {
        let err = Error::CDR(cdr::Error::Message("test".to_string()));
        assert!(format!("{}", err).contains("CDR error"));
    }
}
