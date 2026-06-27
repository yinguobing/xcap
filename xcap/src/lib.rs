use crate::extractor::Extractor;
use crate::parser::{h264, image, odom, pointcloud, timestamp, trailing};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{debug, info, warn};

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

mod extractor;
mod parser;
pub mod storage;
pub mod textlog;

const GROUND_OFFSET: f32 = 0.0;

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
            "[{}] {} msgs: {}, {}, {}",
            self.id,
            self.name,
            if let Some(count) = self.msg_count {
                count.to_string()
            } else {
                "Unknown".to_owned()
            },
            self.format,
            self.description
        )
    }
}

pub fn summary(files: &[PathBuf]) -> Result<Vec<Topic>, Error> {
    // Collect all topics
    let mut topics: HashMap<String, Topic> = HashMap::new();

    // Enumerate all files
    for file in files {
        // Read summary
        let fd = fs::File::open(file)?;
        let mmap = unsafe { memmap2::Mmap::map(&fd)? };
        let summary = match mcap::read::Summary::read(&mmap) {
            Ok(summary) => summary.unwrap(),
            Err(e) => {
                warn!("Failed to read summary from {}: {}", file.display(), e);
                continue;
            }
        };

        // Statistics
        let stats = summary
            .stats
            .ok_or(Error::NoStatistics(file.display().to_string()))?;

        // Topics
        for chn in summary.channels {
            debug!("Channel: {}, {:?}", chn.1.message_encoding, chn.1.metadata);
            topics
                .entry(chn.1.topic.clone())
                .and_modify(|t| {
                    t.id = chn.0;
                    t.name.clone_from(&chn.1.topic);
                    t.format.clone_from(&chn.1.schema.as_ref().unwrap().name);
                    t.description =
                        format!("Encoding: {}", chn.1.schema.as_ref().unwrap().encoding);
                    t.msg_count = if let (Some(count), Some(delta)) =
                        (t.msg_count, stats.channel_message_counts.get(&chn.0))
                    {
                        Some(count + delta)
                    } else {
                        None
                    };
                })
                .or_insert(Topic {
                    id: chn.0,
                    name: chn.1.topic.clone(),
                    format: chn.1.schema.as_ref().unwrap().name.clone(),
                    description: format!(
                        "id: {}, encoding: {}",
                        chn.1.schema.as_ref().unwrap().id,
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

#[allow(clippy::too_many_arguments)]
pub fn dump_n_visualize(
    files: &[PathBuf],
    output_dir: &Option<PathBuf>,
    as_binary: bool,
    target_topics: &[String],
    sigint: Arc<AtomicBool>,
    vis_stream: Option<rerun::RecordingStream>,
    point_cloud_scale: &Option<f32>,
    intensity_scale: &Option<f32>,
    topics_in_mcap: &[Topic],
    time_off: i64,
    time_stop: i64,
    video_header: &Option<String>,
    output_image_format: &Option<String>,
    bars: MultiProgress,
) -> Result<(), Error> {
    // Visualization setup, Randomly select a model.
    let ego = [
        include_bytes!("../assets/luigi/luigi.glb").to_vec(),
        include_bytes!("../assets/british-campact-car/car.glb").to_vec(),
    ][(rand::random::<u8>() % 2) as usize]
        .clone();

    // A flag asset for trailing target visualization
    let flag = include_bytes!("../assets/map-pin/pin.glb").to_vec();
    if let Some(rec) = &vis_stream {
        // Setup view coordinates
        rec.log_static("world", &rerun::ViewCoordinates::FLU())
            .unwrap();

        // Setup EGO model
        rec.log_static(
            "world/ego/vehicle",
            &rerun::Asset3D::from_file_contents(ego, Some(rerun::MediaType::glb())),
        )
        .unwrap();
        rec.log_static(
            "world/ego/vehicle",
            &rerun::Transform3D::from_rotation(rerun::Quaternion::from_xyzw([
                std::f32::consts::FRAC_1_SQRT_2,
                0.0,
                0.0,
                std::f32::consts::FRAC_1_SQRT_2,
            ])),
        )
        .unwrap();
    }

    // Setup a progress bar as this could be a time consuming process.
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let mut bar_handles: HashMap<&str, ProgressBar> = HashMap::new();

    // Create a parser group for all different topics.
    let mut parsers: HashMap<
        &str,
        Box<dyn Extractor<ExtractorError = Box<dyn std::error::Error>>>,
    > = HashMap::new();
    for topic_name in target_topics {
        // Locate corresponding topic
        let topic = topics_in_mcap
            .iter()
            .find(|&t| &t.name == topic_name)
            .ok_or(Error::InvalidTopic(topic_name.clone()))?;

        // Using topic name as output directory path
        let output_dir = output_dir
            .as_ref()
            .map(|p| p.join(PathBuf::from(topic_name.trim_start_matches('/'))));

        // Create parser by topic format
        match topic.format.as_str() {
            "builtin_interfaces/msg/Time" => {
                parsers.insert(
                    topic.name.as_str(),
                    Box::new(timestamp::Parser::new(vis_stream.clone())),
                );
            }
            "sensor_msgs/msg/Image" => {
                parsers.insert(
                    topic.name.as_str(),
                    Box::new(image::Parser::new(&output_dir, vis_stream.clone())),
                );
            }
            "custom_msgs/msg/CompressedImage" => {
                parsers.insert(
                    topic.name.as_str(),
                    Box::new(h264::Parser::new(
                        &output_dir,
                        vis_stream.clone(),
                        as_binary,
                        video_header,
                        output_image_format,
                    )),
                );
            }
            "sensor_msgs/msg/PointCloud2" => {
                parsers.insert(
                    topic_name,
                    Box::new(pointcloud::Parser::new(
                        &output_dir,
                        &vis_stream,
                        point_cloud_scale,
                        intensity_scale,
                        &as_binary,
                    )),
                );
            }
            "geometry_msgs/msg/PointStamped" => {
                parsers.insert(
                    topic.name.as_str(),
                    Box::new(trailing::Parser::new(&output_dir, &vis_stream)),
                );
                if let Some(rec) = &vis_stream {
                    // Show the target marker
                    rec.log_static(
                        topic.name.as_str(),
                        &rerun::Asset3D::from_file_contents(
                            flag.clone(),
                            Some(rerun::MediaType::glb()),
                        ),
                    )
                    .unwrap();
                    // Show the attention zone
                    rec.log_static(
                        "trailing/attention_zone",
                        &rerun::Boxes3D::from_centers_and_half_sizes(
                            [(2.8, 0.0, -GROUND_OFFSET)],
                            [(2.35, 5.0, 0.01)],
                        )
                        .with_radii([0.01])
                        .with_colors([rerun::Color::from_rgb(0, 125, 255)]),
                    )
                    .unwrap();
                }
            }
            "nav_msgs/msg/Odometry" => {
                parsers.insert(
                    topic_name.as_str(),
                    Box::new(odom::Parser::new(&output_dir, vis_stream.clone())),
                );
            }
            _ => {
                return Err(Error::InvalidTopic(format!(
                    "'{}' of format '{}' is not supported yet.",
                    topic_name, topic.format
                )));
            }
        }

        // Init progress bars
        let topic = topics_in_mcap
            .iter()
            .find(|t| &t.name == topic_name)
            .unwrap();
        bar_handles.insert(
            topic_name,
            bars.add(ProgressBar::new(topic.msg_count.unwrap_or(0))),
        );
    }

    // Setup progress bars
    for h in bar_handles.values_mut() {
        h.set_style(sty.clone());
    }

    // Setup blueprints TODO.
    let _enable_3d_view = parsers.iter().any(|p| p.1.generates_3d_data());
    let _enable_2d_view = parsers.iter().any(|p| p.1.generates_2d_data());

    // Enumerate all files
    'loop_file: for file in files.iter() {
        // Read in files
        let fd = fs::File::open(file)?;
        let mmap = unsafe { memmap2::Mmap::map(&fd)? };

        // Enumerate all messages
        for message in mcap::MessageStream::new(&mmap)? {
            // Check for interrupt
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(Error::Interrupted);
            }

            let msg = message?;

            // Trim start/end
            if (msg.publish_time as i64) <= time_off {
                continue;
            }
            if (msg.publish_time as i64) >= time_stop {
                info!("Stop time reached");
                break 'loop_file;
            }

            // Parse message
            let topic_name = msg.channel.topic.as_str();
            let Some(parser) = parsers.get_mut(topic_name) else {
                continue;
            };
            parser
                .step(&msg)
                .map_err(|e| Error::ParserError(e.to_string()))?;
            let bar = bar_handles.get(topic_name).unwrap();
            bar.set_message(topic_name.to_string());
            bar.inc(1);
        }
    }

    // Post process
    for (_, parser) in parsers.iter_mut() {
        parser
            .post_process(sigint.clone())
            .map_err(|e| Error::ParserError(e.to_string()))?;
    }

    Ok(())
}

pub fn trim(
    files: &[PathBuf],
    output_dir: &Path,
    sigint: Arc<AtomicBool>,
    time_off: i64,
    time_stop: i64,
) -> Result<(), Error> {
    // Output file
    let mut trim_out = mcap::Writer::new(std::io::BufWriter::new(fs::File::create(
        output_dir.join("trim.mcap"),
    )?))?;

    // Enumerate all files
    'loop_file: for file in files.iter() {
        // Read in files
        let fd = fs::File::open(file)?;
        let mmap = unsafe { memmap2::Mmap::map(&fd)? };

        // Enumerate all messages
        for message in mcap::MessageStream::new(&mmap)? {
            // Check for interrupt
            if sigint.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(Error::Interrupted);
            }

            let msg = message?;

            // Trim start/end
            if msg.publish_time < time_off as u64 {
                continue;
            }
            if msg.publish_time > time_stop as u64 {
                info!("Stop time reached");
                break 'loop_file;
            }

            trim_out.write(&msg)?;
        }
    }

    trim_out.finish()?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_display_with_msg_count() {
        let topic = Topic {
            id: 42,
            name: "/camera/image".to_string(),
            format: "sensor_msgs/msg/Image".to_string(),
            description: "id: 1, encoding: cdr".to_string(),
            msg_count: Some(100),
        };
        let display = format!("{}", topic);
        assert!(display.contains("42"));
        assert!(display.contains("/camera/image"));
        assert!(display.contains("100"));
        assert!(display.contains("sensor_msgs/msg/Image"));
    }

    #[test]
    fn topic_display_without_msg_count() {
        let topic = Topic {
            id: 1,
            name: "/odom".to_string(),
            format: "nav_msgs/msg/Odometry".to_string(),
            description: "id: 2, encoding: cdr".to_string(),
            msg_count: None,
        };
        let display = format!("{}", topic);
        assert!(display.contains("1"));
        assert!(display.contains("/odom"));
        assert!(display.contains("Unknown"));
    }

    #[test]
    fn error_display() {
        let err = Error::InvalidTopic("test_topic".to_string());
        assert_eq!(format!("{}", err), "Invalid topic. test_topic");

        let err = Error::Interrupted;
        assert_eq!(format!("{}", err), "Interrupted");

        let err = Error::Unknown;
        assert_eq!(format!("{}", err), "unknown error");

        let err = Error::NoSummary("file.mcap".to_string());
        assert!(format!("{}", err).contains("file.mcap"));

        let err = Error::ParserError("bad data".to_string());
        assert!(format!("{}", err).contains("bad data"));
    }
}
