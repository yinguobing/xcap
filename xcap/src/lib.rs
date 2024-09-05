use extractor::Extractor;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{error, info};
use std::sync::{atomic::AtomicBool, Arc};
use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

mod extractor;
mod h264;
mod pointcloud;
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
    #[error("Interrupted")]
    Interrupted,
    #[error("H.264 error. {0}")]
    H264Error(#[from] h264::Error),
    #[error("Failed to parse message. {0}")]
    ParserError(String),
    #[error("unknown error")]
    Unknown,
}

pub struct Topic {
    pub id: u16,
    pub name: String,
    pub format: String,
    pub description: String,
    pub msg_count: Option<u64>,
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, msgs: {}, {}, {}",
            self.id,
            self.name,
            if self.msg_count.is_some() {
                self.msg_count.unwrap().to_string()
            } else {
                "Unknown".to_owned()
            },
            self.format,
            self.description
        )
    }
}

pub fn summary(files: &Vec<PathBuf>) -> Result<Vec<Topic>, Error> {
    // Collect all topics
    let mut topics: HashMap<u16, Topic> = HashMap::new();

    // Enumerate all files
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
                    t.format.clone_from(&chn.1.schema.as_ref().unwrap().name);
                    t.description =
                        format!("Encoding: {}", chn.1.schema.as_ref().unwrap().encoding);
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
                    format: chn.1.schema.as_ref().unwrap().name.clone(),
                    description: format!("Encoding: {}", chn.1.schema.as_ref().unwrap().encoding),
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
    topic_names: &Vec<String>,
    sigint: Arc<AtomicBool>,
    vis_stream: Option<Arc<rerun::RecordingStream>>,
) -> Result<(), Error> {
    // Get all topics
    let topics = summary(files)?;

    // Setup a progress bar as this could be a time consuming process.
    let bars = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    let mut bar_handles: HashMap<&str, ProgressBar> = HashMap::new();
    let mut bar_progress: HashMap<&str, u32> = HashMap::new();

    // Create a parser group for all different topics.
    let mut parsers: HashMap<
        &str,
        Box<dyn Extractor<ExtractorError = Box<dyn std::error::Error>>>,
    > = HashMap::new();
    for topic_name in topic_names {
        // Locate corresponding topic
        let topic = topics
            .iter()
            .find(|x| x.name == *topic_name)
            .ok_or(Error::InvalidTopic(format!(
                "Topic not found: {}",
                topic_name
            )))?;

        // Using topic name as output directory path
        let output_dir = output_dir.join(PathBuf::from(topic_name.trim_start_matches('/')));
        fs::create_dir_all(&output_dir)?;

        // Create parser by topic format
        match topic.format.as_str() {
            "sensor_msgs/msg/CompressedImage" => {
                parsers.insert(
                    topic.name.as_str(),
                    Box::new(h264::Parser::new(&output_dir)),
                );
            }
            "sensor_msgs/msg/PointCloud2" => {
                parsers.insert(
                    topic.name.as_str(),
                    Box::new(pointcloud::Parser::new(&output_dir, vis_stream.clone())),
                );
            }
            _ => {
                return Err(Error::InvalidTopic(format!(
                    "Topic format not supported: {}",
                    topic.format
                )));
            }
        }

        // Init progress bars
        bar_handles.insert(
            topic_name,
            bars.add(ProgressBar::new(topic.msg_count.unwrap_or(0))),
        );
        for h in bar_handles.values_mut() {
            h.set_style(sty.clone());
        }
        bar_progress.insert(topic_name, 0);
    }

    // Enumerate all files
    for file in files.iter() {
        // Read in files
        let mut fd = fs::File::open(file)?;
        let mut buf = Vec::new();
        fd.read_to_end(&mut buf)?;

        // Enumerate all messages
        for message in mcap::MessageStream::new(&buf)? {
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(Error::Interrupted);
            }
            let msg = message?;
            let topic_name = msg.channel.topic.as_str();
            let Some(parser) = parsers.get_mut(topic_name) else {
                continue;
            };

            parser
                .step(&msg)
                .map_err(|e| Error::ParserError(e.to_string()))?;

            bar_progress
                .get_mut(topic_name)
                .unwrap()
                .checked_add(1)
                .unwrap();

            let bar = bar_handles.get(topic_name).unwrap();
            bar.set_message(topic_name.to_string());
            bar.inc(1);
        }
    }

    // Post process
    info!("Post processing...");
    for (name, parser) in parsers.iter_mut() {
        info!("- {}", name);
        parser
            .post_process(sigint.clone())
            .map_err(|e| Error::ParserError(e.to_string()))?;
    }

    Ok(())
}
