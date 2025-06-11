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
    Cdr(#[from] cdr::Error),
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
            .map_err(Error::Cdr)?;

        if let Some(rec) = &self.rec_stream {
            rec.set_timestamp_secs_since_epoch(
                "main",
                stamp.sec as f64 + stamp.nanosec as f64 * 1e-9,
            );

            // CPU
            rec.log(
                message.channel.topic.clone(),
                &rerun::Scalars::new([stamp.sec as f64 + stamp.nanosec as f64 * 1e-9]),
            )
            .unwrap();
        }

        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
