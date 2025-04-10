use clap::{Parser, Subcommand};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{error, info, warn};
use rand::Rng;
use std::sync::atomic::AtomicBool;
use std::{env, fs, path::PathBuf, sync::Arc};
use url::Url;
use xcap::{process, storage::Agent, summary};

struct RuntimeError(String);

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Extract ROS messages from MCAP files.")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract ROS messages from MCAP files.
    Extract {
        /// Input resource. Could be a local directory or a remote S3 URL.
        #[arg(short, long)]
        input: String,

        /// Output directory path.
        #[arg(short, long)]
        output_dir: Option<PathBuf>,

        /// Topics to be extracted, separated by comma. Example: "topic,another/topic,/yet/another/topic"
        #[arg(long, value_delimiter = ' ', num_args = 1..)]
        topics: Option<Vec<String>>,

        /// Scale the point cloud in spatial by this factor in preview. Default: 1.0
        #[arg(long)]
        point_cloud_scale: Option<f32>,

        /// Scale the point cloud intensity by this factor in preview. Default: 1.0
        #[arg(long)]
        intensity_scale: Option<f32>,

        /// Enable preview. Default: false
        #[arg(long, default_value_t = false)]
        preview: bool,

        /// Set the start time offset `HH:MM:SS` in UTC. Default: 00:00:00.
        #[arg(long, default_value_t = String::from("1970-1-1 00:00:00"))]
        time_off: String,

        /// Set the stop time `HH:MM:SS` in UTC. The decoding process will reatch to the end of the file if not specified.
        #[arg(long)]
        time_stop: Option<String>,
    },

    /// Visualize ROS messages from MCAP files.
    Show {
        /// Input resource. Could be a local directory or a remote S3 URL.
        #[arg(short, long)]
        input: String,

        /// Topics to be visualized, separated by comma. Example: "topic,another/topic,/yet/another/topic"
        #[arg(long, value_delimiter = ' ', num_args = 1..)]
        topics: Option<Vec<String>>,

        /// Scale the point cloud by this factor. Default: 1.0
        #[arg(long)]
        point_cloud_scale: Option<f32>,

        /// Scale the point cloud intensity by this factor in preview. Default: 1.0
        #[arg(long)]
        intensity_scale: Option<f32>,

        /// Set the start time offset `YEAR-MONTH-DAY HH:MM:SS` in UTC.
        #[arg(long, default_value_t = String::from("1970-1-1 00:00:00"))]
        time_off: String,

        /// Set the stop time `YEAR-MONTH-DAY HH:MM:SS` in UTC. The decoding process will reatch to the end of the file if not specified.
        #[arg(long)]
        time_stop: Option<String>,
    },

    /// Trim MCAP files.
    Trim {
        /// Input resource. Could be a local directory or a remote S3 URL.
        #[arg(short, long)]
        input: String,

        /// Set the start time offset `YEAR-MONTH-DAY HH:MM:SS` in UTC.
        #[arg(long, default_value_t = String::from("1970-1-1 00:00:00"))]
        time_off: String,

        /// Set the stop time `YEAR-MONTH-DAY HH:MM:SS` in UTC. The decoding process will reatch to the end of the file if not specified.
        #[arg(long)]
        time_stop: Option<String>,
    },
}

/// Prepare inputs. Download from remote server if necessary.
async fn prepare_inputs(
    source: &str,
    download_path: &mut Option<PathBuf>,
    sigint: &Arc<AtomicBool>,
) -> Result<Vec<PathBuf>, RuntimeError> {
    // Safety first
    let mut input_src = source.to_owned();
    if input_src.is_empty() {
        return Err(RuntimeError("Input source is empty.".to_string()));
    }

    // Download from remote server?
    if input_src.starts_with("http") {
        let valid_url =
            Url::parse(&input_src).map_err(|e| RuntimeError(format!("Invalid URL. {}", e)))?;

        let base_url = format!(
            "{}://{}:{}",
            valid_url.scheme(),
            valid_url
                .host_str()
                .ok_or(RuntimeError(format!("URL host is None.")))?,
            valid_url
                .port()
                .ok_or(RuntimeError(format!("URL port is None.")))?,
        );
        let bucket = valid_url
            .path_segments()
            .ok_or(RuntimeError(format!("Invalid URL path.")))?
            .next()
            .ok_or(RuntimeError(format!("Failed to get bucket name.")))?;
        let obj_name = valid_url
            .path_segments()
            .unwrap()
            .last()
            .ok_or(RuntimeError(format!("Failed to get object name.")))?;
        let object_dir = valid_url
            .path()
            .trim_start_matches('/')
            .trim_start_matches(bucket)
            .trim_start_matches('/')
            .trim_end_matches(obj_name)
            .trim_end_matches('/');

        let region = env::var("S3_REGION")
            .map_err(|_| RuntimeError("Environment variable `S3_REGION` not set.".to_string()))?;
        let access_key = env::var("S3_ACCESS_KEY").map_err(|_| {
            RuntimeError("Environment variable `S3_ACCESS_KEY` not set.".to_string())
        })?;
        let secret_key = env::var("S3_SECRET_KEY").map_err(|_| {
            RuntimeError("Environment variable `S3_SECRET_KEY` not set.".to_string())
        })?;
        let storage = Agent::new(&base_url, &region, &access_key, &secret_key)
            .map_err(|e| RuntimeError(format!("Storage init failed. {}", e)))?;

        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        const STR_LEN: usize = 6;
        let mut rng = rand::thread_rng();
        let rand_str: String = (0..STR_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        let _down_path = PathBuf::from(format!("/tmp/{}-{}", bucket, rand_str));
        std::fs::create_dir_all(&_down_path).map_err(|e| {
            RuntimeError(format!(
                "Failed to create download directory: {}, {}",
                _down_path.display(),
                e
            ))
        })?;
        download_path.clone_from(&Some(_down_path.clone()));

        info!("Downloading from bucket: {}", bucket);
        storage
            .download_dir(bucket, object_dir, &_down_path, sigint)
            .await
            .map_err(|e| RuntimeError(format!("Download failed. {}", e)))?;

        input_src = _down_path
            .to_str()
            .ok_or(RuntimeError(format!(
                "Get OS string failed. {}",
                _down_path.display()
            )))?
            .to_string();
    } else {
        input_src = source.to_owned();
    }

    let input_dir = PathBuf::from(input_src);
    if !input_dir.exists() {
        return Err(RuntimeError(format!(
            "Input directory not found: {}",
            input_dir.display()
        )));
    }

    // Find all MCAP files
    let mut files: Vec<PathBuf> = fs::read_dir(&input_dir)
        .map_err(|e| RuntimeError(format!("Failed to read directory: {}", e)))?
        .map(|f| f.unwrap().path())
        .filter(|f| f.is_file() && f.extension().is_some_and(|f| f.eq("mcap")))
        .collect();
    files.sort();

    Ok(files)
}

fn cleanup(local_path: &Option<PathBuf>) {
    if let Some(path) = local_path {
        match std::fs::remove_dir_all(path) {
            Ok(_) => {
                info!("Temp directory cleaned.");
            }
            Err(e) => {
                error!("Failed to remove directory: {}. {}", path.display(), e);
            }
        }
    }
}

fn make_rerun_stream() -> (
    rerun::RecordingStream,
    Option<rerun::sink::MemorySinkStorage>,
) {
    let builder = rerun::RecordingStreamBuilder::new("XCAP");
    let (stream, storage) = if cfg!(feature = "native_viewer") {
        let (strm, stor) = builder.memory().expect("Rerun should be initialized.");
        (strm, Some(stor))
    } else {
        let strm = builder.spawn().expect("Rerun should be spawned");
        (strm, None)
    };
    return (stream, storage);
}

#[tokio::main]
async fn main() {
    // Initialization
    let mut download_path: Option<PathBuf> = None;
    let sigint = Arc::new(AtomicBool::new(false));

    // Progress bar setup
    let bars = MultiProgress::new();

    // Logger setup
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let level = logger.filter();
    indicatif_log_bridge::LogWrapper::new(bars.clone(), logger)
        .try_init()
        .unwrap();
    log::set_max_level(level);

    // Catch SIGINT
    let handler_sigint = sigint.clone();
    ctrlc::set_handler(move || {
        warn!("received Ctrl+C! Mission aborted by user.");
        handler_sigint.store(true, std::sync::atomic::Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    // Parse user args
    let cli = Cli::parse();
    let (
        input,
        output_dir,
        topics,
        visualize,
        dump_data,
        point_cloud_scale,
        intensity_scale,
        time_off,
        time_stop,
    ) = match &cli.command {
        Commands::Extract {
            input,
            output_dir,
            topics,
            preview,
            point_cloud_scale,
            intensity_scale,
            time_off,
            time_stop,
        } => (
            input,
            output_dir,
            topics,
            preview,
            true,
            *point_cloud_scale,
            *intensity_scale,
            time_off,
            time_stop,
        ),
        Commands::Show {
            input,
            topics,
            point_cloud_scale,
            intensity_scale,
            time_off,
            time_stop,
        } => (
            input,
            &None,
            topics,
            &true,
            false,
            *point_cloud_scale,
            *intensity_scale,
            time_off,
            time_stop,
        ),
        Commands::Trim {
            input,
            time_off,
            time_stop,
        } => (
            input, &None, &None, &true, false, None, None, time_off, time_stop,
        ),
    };

    // Prepare inputs
    let files = match prepare_inputs(&input, &mut download_path, &sigint).await {
        Ok(f) => f,
        Err(e) => {
            error!("{}", e.0);
            cleanup(&download_path);
            return;
        }
    };
    if sigint.load(std::sync::atomic::Ordering::Relaxed) {
        cleanup(&download_path);
        return;
    }
    if files.is_empty() {
        error!("No MCAP files found.");
        cleanup(&download_path);
        return;
    }
    info!("Found MCAP files: {}", files.len());
    for f in files.iter() {
        info!("- {}", f.display());
    }

    // Summary this job, this will log useful info such as topics for user.
    let topics_in_mcap = match summary(&files) {
        Ok(t) => t,
        Err(e) => {
            error!("{}", e);
            cleanup(&download_path);
            return;
        }
    };
    info!("Found topics: {}", topics_in_mcap.len());
    for topic in topics_in_mcap.iter() {
        info!("- {}", topic);
    }

    // Output directory
    let output_dir = output_dir
        .clone()
        .unwrap_or(std::env::current_dir().unwrap());
    if dump_data {
        info!("Output directory: {}", output_dir.display());
    }

    // Start time and stop time
    let start_time = match chrono::NaiveDateTime::parse_from_str(&time_off, "%Y-%m-%d %H:%M:%S") {
        Ok(t) => t.and_utc().timestamp_nanos_opt().unwrap(),
        Err(e) => {
            error!("Parse start time failed, {}", e);
            cleanup(&download_path);
            return;
        }
    };
    let stop_time = if time_stop.is_none() {
        i64::MAX
    } else {
        match chrono::NaiveDateTime::parse_from_str(
            time_stop.as_ref().unwrap(),
            "%Y-%m-%d %H:%M:%S",
        ) {
            Ok(t) => t.and_utc().timestamp_nanos_opt().unwrap(),
            Err(e) => {
                error!("Parse stop time failed, {}", e);
                cleanup(&download_path);
                return;
            }
        }
    };

    // Trim only mode?
    let trim_only = match &cli.command {
        Commands::Trim { .. } => true,
        _ => false,
    };

    // Check target topics to make sure they make sense for extraction and
    // visualization. Trim does not need this.
    let mut target_topics: Vec<String> = vec![];

    if !trim_only {
        let Some(topic_str) = topics else {
            error!("No topic specified. Use `--topics` to set topics.");
            cleanup(&download_path);
            return;
        };
        target_topics = topic_str.clone();
        if target_topics.is_empty() {
            error!("No topic specified. Use `--topics` to set topics.");
            cleanup(&download_path);
            return;
        }
        for topic_name in target_topics.iter() {
            let Some(_) = topics_in_mcap.iter().find(|t| t.name == *topic_name) else {
                error!("Topic not found: {}", topic_name);
                cleanup(&download_path);
                return;
            };
        }
    }

    // Visualize required?
    let (rerun_stream, storage) = if *visualize && !trim_only {
        let (stm, sto) = make_rerun_stream();
        (Some(stm), sto)
    } else {
        (None, None)
    };

    // Progress spinner
    let bar_style = ProgressStyle::with_template("{spinner:.blue} {msg}").unwrap();
    let bar = ProgressBar::new_spinner()
        .with_message("Processing...")
        .with_style(bar_style);
    bar.enable_steady_tick(std::time::Duration::from_millis(100));
    let pb = bars.add(bar);

    // Process
    let ret = process(
        &files,
        &output_dir,
        &target_topics,
        sigint,
        rerun_stream,
        dump_data,
        point_cloud_scale,
        intensity_scale,
        topics_in_mcap,
        start_time,
        stop_time,
        trim_only,
        bars,
    );

    // Cleanup
    pb.finish_and_clear();
    cleanup(&download_path);

    // Will block program execution!
    if cfg!(feature = "native_viewer") {
        let _ = rerun::native_viewer::show(
            rerun::MainThreadToken::i_promise_i_am_on_the_main_thread(),
            storage.expect("Rerun storage should be ready.").take(),
        );
    }

    // Take aways
    match ret {
        Ok(_) => {
            info!("Done.");
        }
        Err(e) => {
            error!("{}", e);
            warn!("Sorry, job failed.");
        }
    }
}
