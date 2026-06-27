#![allow(clippy::upper_case_acronyms)]
use crate::extractor::Extractor;
use mcap::Message;
use rerun::RecordingStream;
use ros2_interfaces_humble::geometry_msgs::msg::PointStamped;
use std::{
    fs,
    io::Write,
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

pub struct Parser {
    // Output directory
    output_dir: Option<PathBuf>,

    // Visualizer with rerun
    rec_stream: Option<RecordingStream>,
}

impl Parser {
    pub fn new(output_path: &Option<PathBuf>, rerun_stream: &Option<RecordingStream>) -> Self {
        // Create output dir
        if let Some(output_path) = output_path {
            fs::create_dir_all(output_path).unwrap();
        }

        Parser {
            output_dir: output_path.clone(),
            rec_stream: rerun_stream.clone(),
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let serialized = self.decode(message)?;
        let point_stamped =
            cdr::deserialize_from::<_, PointStamped, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(Error::CDR)?;

        if let Some(rec) = &self.rec_stream {
            let x = point_stamped.point.x as f32;
            let y = point_stamped.point.y as f32;
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
            rec.set_timestamp_secs_since_epoch(
                "msg_header",
                point_stamped.header.stamp.sec as f64
                    + point_stamped.header.stamp.nanosec as f64 * 1e-9,
            );
            rec.log(
                format!(
                    "world/ego/{}",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Transform3D::from_translation_rotation_scale(
                    rerun::Vec3D::from([x - 0.6, y, 1.32]),
                    rerun::Quaternion::from_xyzw([0.5, 0.5, 0.5, 0.5]),
                    rerun::Scale3D::from(0.1),
                ),
            )
            .unwrap();
        }

        // Create output file
        if let Some(output_dir) = &self.output_dir {
            let mut file =
                fs::File::create(output_dir.join(format!("{}.bin", message.publish_time)))?;
            file.write_all(&message.data)?;
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
    fn parser_creation_without_output() {
        let parser = Parser::new(&None, &None);
        assert!(!parser.generates_2d_data());
        assert!(parser.generates_3d_data());
    }

    #[test]
    fn error_display() {
        let err = Error::CDR(cdr::Error::Message("test".to_string()));
        assert!(format!("{}", err).contains("CDR error"));
    }
}
