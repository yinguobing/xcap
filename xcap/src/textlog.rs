use regex::Regex;
use rerun::RecordingStream;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::{atomic::AtomicBool, Arc};

use crate::textlog;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interrupted")]
    Interrupted,
    #[error("IO Error. {0}")]
    IO(#[from] std::io::Error),
    #[error("Regrex Error. {0}")]
    Regex(#[from] regex::Error),
    #[error("Rerun Error. {0}")]
    Rerun(#[from] rerun::RecordingStreamError),
}

pub struct Parser {
    // Text log file
    log_files: Vec<PathBuf>,

    // Visualizer with rerun
    rec_stream: Option<RecordingStream>,
}

impl Parser {
    pub fn new(rerun_stream: Option<RecordingStream>, log_files: &[PathBuf]) -> Self {
        Parser {
            rec_stream: rerun_stream,
            log_files: log_files.to_owned(),
        }
    }

    pub fn parse(&self, stop_flag: Arc<AtomicBool>) -> Result<(), textlog::Error> {
        let re = Regex::new(r"(?P<ts>\d{10}(?:\.\d+)?)")?;

        for log_file in &self.log_files {
            let file = File::open(log_file)?;
            let reader = BufReader::new(file);
            let file_name = log_file
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("log");
            if let Some(rec) = &self.rec_stream {
                for line in reader.lines() {
                    if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                        return Err(Error::Interrupted);
                    }
                    let line = line?;
                    // Find the first unix epoch timestamp in the line
                    if let Some(caps) = re.captures(&line) {
                        if let Some(ts) = caps.name("ts") {
                            let ts_f64: f64 = ts.as_str().parse().unwrap_or(0.0);
                            rec.set_timestamp_secs_since_epoch("ros_publish", ts_f64);
                            rec.set_timestamp_secs_since_epoch("ros_log", ts_f64);
                            rec.set_timestamp_secs_since_epoch("msg_header", ts_f64);
                            rec.log(
                                format!("log/{}", file_name),
                                &rerun::TextLog::new(line.clone()),
                            )?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_extracts_timestamp_from_line() {
        let re = Regex::new(r"(?P<ts>\d{10}(?:\.\d+)?)").unwrap();

        let line = "[1718000000.123] Some log message";
        let caps = re.captures(line).unwrap();
        assert_eq!(caps.name("ts").unwrap().as_str(), "1718000000.123");

        let line = "1718000000 extra text";
        let caps = re.captures(line).unwrap();
        assert_eq!(caps.name("ts").unwrap().as_str(), "1718000000");
    }

    #[test]
    fn regex_no_timestamp_in_line() {
        let re = Regex::new(r"(?P<ts>\d{10}(?:\.\d+)?)").unwrap();

        let line = "no timestamp here";
        assert!(re.captures(line).is_none());
    }

    #[test]
    fn regex_extracts_first_timestamp_only() {
        let re = Regex::new(r"(?P<ts>\d{10}(?:\.\d+)?)").unwrap();

        let line = "1718000000.5 and 1718000001.0 later";
        let caps = re.captures(line).unwrap();
        assert_eq!(caps.name("ts").unwrap().as_str(), "1718000000.5");
    }

    #[test]
    fn regex_timestamp_edge_cases() {
        let re = Regex::new(r"(?P<ts>\d{10}(?:\.\d+)?)").unwrap();

        // Exactly 10 digits
        assert!(re.captures("0000000000").is_some());
        // 9 digits - no match
        assert!(re.captures("123456789").is_none());
        // 11 digits without dot - matches first 10
        let caps = re.captures("12345678901").unwrap();
        assert_eq!(caps.name("ts").unwrap().as_str(), "1234567890");
        // With fractional part
        let caps = re.captures("1234567890.123456").unwrap();
        assert_eq!(caps.name("ts").unwrap().as_str(), "1234567890.123456");
    }
}
