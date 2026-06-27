#![allow(clippy::upper_case_acronyms)]
use crate::extractor::Extractor;
use mcap::Message;
use rerun::RecordingStream;
use ros2_interfaces_humble::builtin_interfaces::msg::Time;
use std::sync::{atomic::AtomicBool, Arc};

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
}

impl Parser {
    pub fn new(rerun_stream: Option<RecordingStream>) -> Self {
        Parser {
            rec_stream: rerun_stream,
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let serialized = self.decode(message)?;
        let stamp = cdr::deserialize_from::<_, Time, _>(serialized.as_slice(), cdr::size::Infinite)
            .map_err(Error::CDR)?;

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
            rec.set_timestamp_secs_since_epoch(
                "msg_header",
                stamp.sec as f64 + stamp.nanosec as f64 * 1e-9,
            );

            // CPU
            rec.log(
                message.channel.topic.trim_start_matches("/"),
                &rerun::Scalars::new([stamp.sec as f64 + stamp.nanosec as f64 * 1e-9]),
            )
            .unwrap();
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
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creation() {
        let parser = Parser::new(None);
        assert!(!parser.generates_2d_data());
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
