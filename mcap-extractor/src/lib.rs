use extractor::Extractor;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
use std::sync::{atomic::AtomicBool, Arc};
use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    time::Duration,
};

mod extractor;
mod h264;
pub mod storage;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read summary info: {0}")]
    NoSummary(String),
    #[error("Failed to read statistics info: {0}")]
    NoStatistics(String),
    #[error("Invalid topic. {0}")]
    InvalidTopic(String),
    #[error("McapError. {0}")]
    McapError(#[from] mcap::McapError),
    #[error("IO error. {0}")]
    IOError(#[from] io::Error),
    #[error("Interupted")]
    Interupted,
    #[error("H.264 error. {0}")]
    H264Error(#[from] h264::Error),
    #[error("unknown error")]
    Unknown,
}

pub struct Topic {
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

pub fn summary(files: &Vec<PathBuf>) -> Result<Vec<Topic>, Error> {
    // Collect all topics
    let mut topics: HashMap<u16, Topic> = HashMap::new();

    // Enemerate all files
    for file in files {
        // Read summary
        let mut fd = fs::File::open(file)?;
        let mut buf = Vec::new();
        fd.read_to_end(&mut buf)?;
        let summary =
            mcap::read::Summary::read(&buf)?.ok_or(Error::NoSummary(file.display().to_string()))?;

        // Statistics
        let stats = summary
            .stats
            .ok_or(Error::NoStatistics(file.display().to_string()))?;

        // Topics
        for chn in summary.channels {
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
    let mut topics: Vec<Topic> = topics.into_values().collect();
    topics.sort_by_key(|k| k.id);
    Ok(topics)
}

pub fn process(
    files: &Vec<PathBuf>,
    output_dir: &Path,
    topic_name: &Option<String>,
    sigint: Arc<AtomicBool>,
) -> Result<(), Error> {
    // Get all topics
    let topics = summary(files)?;

    // Topic name valid?
    let topic_name = topic_name.as_ref().ok_or(Error::InvalidTopic(
        "No topic specified. Use `--topic` to set topic.".to_string(),
    ))?;

    let Some(_) = topics.iter().find(|t| &t.name == topic_name) else {
        return Err(Error::InvalidTopic(topic_name.clone()));
    };

    info!("- Extracting H.264...");

    // Setup a progress bar.
    let spinner_style = ProgressStyle::default_spinner()
        .template("{prefix:} {spinner} {wide_msg}")
        .unwrap();
    let bar = ProgressBar::new_spinner();
    bar.set_style(spinner_style);
    bar.enable_steady_tick(Duration::from_millis(100));

    // Using topic name as directory path
    let sub_dir = PathBuf::from(topic_name.trim_start_matches('/'));
    let output_dir = output_dir.join(sub_dir);
    fs::create_dir_all(&output_dir)?;

    let mut parser = h264::Parser::new(&output_dir);
    let mut counter = 0;
    for file in files.iter() {
        let mut fd = fs::File::open(file)?;
        let mut buf = Vec::new();
        fd.read_to_end(&mut buf)?;
        let stream = mcap::MessageStream::new(&buf)?
            .filter(|x| x.as_ref().is_ok_and(|x| &x.channel.topic == topic_name));
        for message in stream {
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(Error::Interupted);
            }
            parser.step(&message?)?;
            counter += 1;
            bar.set_message(format!("{} {}", topic_name, counter));
        }
    }
    bar.finish();

    // Post process
    info!("- Extracting frames...");
    parser.post_process(sigint)?;

    Ok(())
}
