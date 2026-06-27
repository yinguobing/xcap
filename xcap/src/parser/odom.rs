#![allow(clippy::upper_case_acronyms)]
use crate::extractor::Extractor;
use mcap::Message;
use ndarray::arr2;
use ndarray_linalg::solve::Inverse;
use rerun::RecordingStream;
use ros2_interfaces_humble::nav_msgs::msg::Odometry;
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
    // Visualizer with rerun
    rec_stream: Option<RecordingStream>,

    // Output directory
    output_dir: Option<PathBuf>,
}

impl Parser {
    pub fn new(output_path: &Option<PathBuf>, rerun_stream: Option<RecordingStream>) -> Self {
        // Create output dir
        if let Some(output_path) = output_path {
            fs::create_dir_all(output_path).unwrap();
        }

        Parser {
            rec_stream: rerun_stream,
            output_dir: output_path.clone(),
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let serialized = self.decode(message)?;
        let msg_odom =
            cdr::deserialize_from::<_, Odometry, _>(serialized.as_slice(), cdr::size::Infinite)
                .map_err(Error::CDR)?;

        // Timestamp
        let timestamp =
            msg_odom.header.stamp.sec as f64 + msg_odom.header.stamp.nanosec as f64 * 1e-9;

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

            // Odometry
            rec.log(
                format!(
                    "position/{}/x",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Scalars::single(msg_odom.pose.pose.position.x),
            )?;
            rec.log(
                format!(
                    "position/{}/y",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Scalars::single(msg_odom.pose.pose.position.y),
            )?;
            rec.log(
                format!(
                    "position/{}/z",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Scalars::single(msg_odom.pose.pose.position.z),
            )?;
            rec.log(
                format!(
                    "quaternion/{}/x",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Scalars::single(msg_odom.pose.pose.orientation.x),
            )?;
            rec.log(
                format!(
                    "quaternion/{}/y",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Scalars::single(msg_odom.pose.pose.orientation.y),
            )?;
            rec.log(
                format!(
                    "quaternion/{}/z",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Scalars::single(msg_odom.pose.pose.orientation.z),
            )?;
            rec.log(
                format!(
                    "quaternion/{}/w",
                    &message.channel.topic.trim_start_matches("/")
                ),
                &rerun::Scalars::single(msg_odom.pose.pose.orientation.w),
            )?;

            let imu2body: ndarray::ArrayBase<
                ndarray::OwnedRepr<f32>,
                ndarray::Dim<[usize; 2]>,
                f32,
            > = arr2(&[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, -1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);
            let q: quaternion_core::Quaternion<f32> = (
                msg_odom.pose.pose.orientation.w as f32,
                [
                    msg_odom.pose.pose.orientation.x as f32,
                    msg_odom.pose.pose.orientation.y as f32,
                    msg_odom.pose.pose.orientation.z as f32,
                ],
            );
            let dcm = quaternion_core::to_dcm(q);
            let pose = arr2(&[
                [
                    dcm[0][0],
                    dcm[0][1],
                    dcm[0][2],
                    msg_odom.pose.pose.position.x as f32,
                ],
                [
                    dcm[1][0],
                    dcm[1][1],
                    dcm[1][2],
                    msg_odom.pose.pose.position.y as f32,
                ],
                [
                    dcm[2][0],
                    dcm[2][1],
                    dcm[2][2],
                    msg_odom.pose.pose.position.z as f32,
                ],
                [0.0, 0.0, 0.0, 0.0],
            ]);
            let (p, _) = imu2body
                .inv()
                .unwrap()
                .dot(&pose)
                .dot(&imu2body)
                .into_raw_vec_and_offset();
            let transform3d = rerun::Transform3D::new()
                .with_translation([p[3], p[7], p[11]])
                .with_mat3x3(rerun::Mat3x3::from([
                    p[0], p[4], p[8], p[1], p[5], p[9], p[2], p[6], p[10],
                ]));

            rec.log("world/ego", &transform3d)?;
        }

        // Create output file
        if let Some(output_dir) = &self.output_dir {
            // Output binary file
            let timestamp_as_str = format!("{}", (timestamp * 1000.0) as u64);

            // Output JSON file
            let output_file = output_dir.join(format!("{}.json", timestamp_as_str));
            let mut file = fs::File::create(&output_file)?;
            file.write_all(serde_json::to_string_pretty(&msg_odom).unwrap().as_bytes())?;
        }
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }

    fn generates_3d_data(&self) -> bool {
        false
    }

    fn generates_2d_data(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creation_without_output() {
        let parser = Parser::new(&None, None);
        assert!(parser.generates_2d_data());
        assert!(!parser.generates_3d_data());
    }

    #[test]
    fn error_display() {
        let err = Error::CDR(cdr::Error::Message("test".to_string()));
        assert!(format!("{}", err).contains("CDR error"));

        let io_err = std::io::Error::other("test");
        let err = Error::Zstd(io_err);
        assert!(format!("{}", err).contains("ZSTD error"));
    }
}
