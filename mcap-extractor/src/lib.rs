use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{atomic::AtomicBool, Arc};
use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    time::Duration,
};
use thiserror::Error;

mod h264;
pub mod storage;

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("Failed to read summary info: {0}")]
    NoSummary(String),
    #[error("Failed to read statistics info: {0}")]
    NoStatistics(String),
    #[error("Invalid topic. {0}")]
    InvalidTopic(String),
    #[error("Failed to create directory. {0}")]
    OutputIOError(#[from] io::Error),
    #[error("Interupted")]
    Interupted,
    #[error("H.264 error. {0}")]
    H264Error(#[from] h264::Error),
    #[error("unknown error")]
    Unknown,
}

struct Topic {
    id: u16,
    name: String,
    description: String,
    msg_count: Option<u64>,
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, msgs: {}, {}",
            self.id,
            self.name,
            if self.msg_count.is_some() {
                self.msg_count.unwrap().to_string()
            } else {
                "Unknown".to_owned()
            },
            self.description
        )
    }
}

fn summary(files: &Vec<PathBuf>) -> Result<Vec<Topic>, ExtractorError> {
    let mut topics: HashMap<u16, Topic> = HashMap::new();

    for file in files {
        let mut fd = fs::File::open(file).unwrap();
        let mut buf = Vec::new();
        fd.read_to_end(&mut buf).unwrap();
        let Some(summary) = mcap::read::Summary::read(&buf).unwrap() else {
            return Err(ExtractorError::NoSummary(file.display().to_string()));
        };

        // Statistics
        let Some(stats) = summary.stats else {
            return Err(ExtractorError::NoStatistics(file.display().to_string()));
        };

        // Topics
        let channels: Vec<_> = summary.channels.into_iter().collect();
        for chn in channels {
            topics
                .entry(chn.0)
                .and_modify(|t| {
                    t.id = chn.0;
                    t.name.clone_from(&chn.1.topic);
                    t.description = format!(
                        "MsgType: {}, Encoding: {}",
                        chn.1.schema.as_ref().unwrap().name,
                        chn.1.schema.as_ref().unwrap().encoding
                    );
                    t.msg_count = if t.msg_count.is_some()
                        && stats.channel_message_counts.contains_key(&chn.0)
                    {
                        Some(
                            t.msg_count.unwrap()
                                + stats.channel_message_counts.get(&chn.0).unwrap(),
                        )
                    } else {
                        None
                    };
                })
                .or_insert(Topic {
                    id: chn.0,
                    name: chn.1.topic.clone(),
                    description: format!(
                        "MsgType: {}, Encoding: {}",
                        chn.1.schema.as_ref().unwrap().name,
                        chn.1.schema.as_ref().unwrap().encoding
                    ),
                    msg_count: stats.channel_message_counts.get(&chn.0).copied(),
                });
        }
    }
    Ok(topics.into_values().collect())
}

pub fn process(
    files: &Vec<PathBuf>,
    output_dir: &Path,
    topic_name: &Option<String>,
    sigint: Arc<AtomicBool>,
) -> Result<(), ExtractorError> {
    // Summary these files
    let mut topics = summary(files)?;
    topics.sort_by_key(|k| k.id);
    println!("Found topics: {}", topics.len());
    for topic in topics.iter() {
        println!("- {}", topic);
    }

    // Topic name valid?
    let Some(topic_name) = topic_name.clone() else {
        return Err(ExtractorError::InvalidTopic(
            "No topic specified. Use `--topic` to set topic.".to_string(),
        ));
    };
    let Some(_) = topics.iter().find(|t| t.name == topic_name) else {
        return Err(ExtractorError::InvalidTopic(topic_name));
    };

    // Setup a progress bar.
    let spinner_style = ProgressStyle::default_spinner()
        .template("{prefix:} {spinner} {wide_msg}")
        .unwrap();
    let bar = ProgressBar::new_spinner();
    bar.set_style(spinner_style);
    bar.enable_steady_tick(Duration::from_millis(100));

    println!("Extracting...");

    // Using topic name as directory path
    let sub_dir = PathBuf::from(topic_name.trim_start_matches('/'));
    let output_dir = output_dir.join(sub_dir);
    fs::create_dir_all(&output_dir)?;
    println!("- Output directory: {}", output_dir.display());

    println!("- Extracting H.264...");
    let mut parser = h264::Parser::new(&output_dir);
    let mut counter = 0;
    for file in files.iter() {
        let mut fd = fs::File::open(file).unwrap();
        let mut buf = Vec::new();
        fd.read_to_end(&mut buf).unwrap();
        let stream = mcap::MessageStream::new(&buf)
            .unwrap()
            .filter(|x| x.as_ref().is_ok_and(|x| x.channel.topic == topic_name));
        for message in stream {
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(ExtractorError::Interupted);
            }
            let message = message.unwrap();
            parser.accumulate(&message);
            counter += 1;
            bar.set_message(format!("{} {}", topic_name, counter));
        }
    }
    bar.finish();

    // Dump frames
    println!("- Extracting frames...");
    parser.dump_frames(sigint)?;

    Ok(())
}
