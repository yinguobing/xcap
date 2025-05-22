use clap::{Parser, Subcommand};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use std::sync::atomic::AtomicBool;
use std::{env, fs, path::PathBuf, sync::Arc};
use url::Url;
use xcap::{dump_n_visualize, storage::Agent, summary, trim};

struct RuntimeError(String);

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Extract ROS messages from MCAP files.")]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Input resource. Could be a local directory or a remote S3 URL.
    #[arg(short, long)]
    input: String,

    /// Topics to be extracted, separated by comma. Example: "topic,another/topic,/yet/another/topic"
    #[arg(long, value_delimiter = ' ', num_args = 1..)]
    topics: Option<Vec<String>>,

    /// Scale the point cloud in spatial by this factor in preview. Default: 1.0
    #[arg(long)]
    point_cloud_scale: Option<f32>,

    /// Scale the point cloud intensity by this factor in preview. Default: 1.0
    #[arg(long)]
    intensity_scale: Option<f32>,

    /// Set the start time offset `HH:MM:SS` in UTC. Default: 00:00:00.
    #[arg(long, default_value_t = String::from("1970-1-1 00:00:00"))]
    time_off: String,

    /// Set the stop time `HH:MM:SS` in UTC. The decoding process will reatch to the end of the file if not specified.
    #[arg(long)]
    time_stop: Option<String>,

    /// Rerun viewer URL.
    #[arg(long)]
    viewer_url: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Visualize ROS messages from MCAP files.
    Show,

    /// Extract ROS messages from MCAP files.
    Extract {
        /// Output directory path.
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Enable preview. Default: false
        #[arg(long, default_value_t = false)]
        preview: bool,
    },

    /// Trim MCAP files.
    Trim {
        /// Output directory path.
        #[arg(short, long)]
        output_dir: PathBuf,
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
                .ok_or(RuntimeError("URL host is None.".to_string()))?,
            valid_url
                .port()
                .ok_or(RuntimeError("URL port is None.".to_string()))?,
        );
        let bucket = valid_url
            .path_segments()
            .ok_or(RuntimeError("Invalid URL path.".to_string()))?
            .next()
            .ok_or(RuntimeError("Failed to get bucket name.".to_string()))?;
        let obj_name = valid_url
            .path_segments()
            .unwrap()
            .next_back()
            .ok_or(RuntimeError("Failed to get object name.".to_string()))?;
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

        println!("Downloading from bucket: {}", bucket);
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
                println!("Temp directory cleaned.");
            }
            Err(e) => {
                println!("Failed to remove directory: {}. {}", path.display(), e);
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
    (stream, storage)
}

#[tokio::main]
async fn main() {
    // Logger setup
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error"))
            .build();
    let level = logger.filter();
    log::set_max_level(level);

    // Catch SIGINT
    let sigint = Arc::new(AtomicBool::new(false));
    let handler_sigint = sigint.clone();
    ctrlc::set_handler(move || {
        println!("Ctrl-C received");
        handler_sigint.store(true, std::sync::atomic::Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    // Parse common args
    let args = Args::parse();

    // Prepare input files
    let input = &args.input;
    let mut download_path: Option<PathBuf> = None;
    let files = match prepare_inputs(input, &mut download_path, &sigint).await {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e.0);
            cleanup(&download_path);
            return;
        }
    };
    if sigint.load(std::sync::atomic::Ordering::Relaxed) {
        cleanup(&download_path);
        return;
    }
    if files.is_empty() {
        println!("No MCAP files found.");
        cleanup(&download_path);
        return;
    }
    println!("Found MCAP files: {}", files.len());
    for f in files.iter() {
        println!("- {}", f.display());
    }

    // Summary this job, this will log useful info such as topics for user.
    let topics_in_mcap = match summary(&files) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            cleanup(&download_path);
            return;
        }
    };
    println!("Found topics: {}", topics_in_mcap.len());
    topics_in_mcap.iter().for_each(|t| println!("- {}", t));

    // Check target topics to make sure they make sense for extraction and
    // visualization. Trim does not need this.
    let target_topics = match &args.command {
        Commands::Extract { .. } | Commands::Show => {
            if args.topics.is_none() {
                println!("No topics specified.");
                cleanup(&download_path);
                return;
            }
            for topic_name in args.topics.as_ref().unwrap().iter() {
                if !topics_in_mcap.iter().any(|t| &t.name == topic_name) {
                    println!("Topic not found: {}", topic_name);
                    cleanup(&download_path);
                    return;
                }
            }
            args.topics
        }
        Commands::Trim { .. } => None,
    };

    // Notice user about the output directory
    let output_dir = match &args.command {
        Commands::Extract { output_dir, .. } | Commands::Trim { output_dir } => {
            println!("Output directory: {}", output_dir.display());
            Some(output_dir.clone())
        }
        _ => None,
    };

    // Start time and stop time
    let time_off = match chrono::NaiveDateTime::parse_from_str(&args.time_off, "%Y-%m-%d %H:%M:%S")
    {
        Ok(t) => t.and_utc().timestamp_nanos_opt().unwrap(),
        Err(e) => {
            println!("Parse start time failed, {}", e);
            cleanup(&download_path);
            return;
        }
    };
    let time_stop = if args.time_stop.is_none() {
        i64::MAX
    } else {
        match chrono::NaiveDateTime::parse_from_str(&args.time_stop.unwrap(), "%Y-%m-%d %H:%M:%S") {
            Ok(t) => t.and_utc().timestamp_nanos_opt().unwrap(),
            Err(e) => {
                println!("Parse stop time failed, {}", e);
                cleanup(&download_path);
                return;
            }
        }
    };

    // Visualize required?
    let (vis_stream, storage) = match args.command {
        Commands::Extract { preview: true, .. } | Commands::Show => {
            let (stm, sto) = make_rerun_stream();
            (Some(stm), sto)
        }
        _ => (None, None),
    };

    // Progress bar setup
    let bars = MultiProgress::new();
    indicatif_log_bridge::LogWrapper::new(bars.clone(), logger)
        .try_init()
        .unwrap();
    let bar_style = ProgressStyle::with_template("{spinner} {msg}").unwrap();
    let bar = ProgressBar::new_spinner()
        .with_message("Processing...")
        .with_style(bar_style);
    bar.enable_steady_tick(std::time::Duration::from_millis(100));
    let pb = bars.add(bar);

    // Process
    let ret = match args.command {
        Commands::Extract { .. } | Commands::Show => dump_n_visualize(
            &files,
            &output_dir,
            &target_topics,
            sigint,
            vis_stream,
            &args.point_cloud_scale,
            &args.intensity_scale,
            &topics_in_mcap,
            time_off,
            time_stop,
            bars,
        ),
        Commands::Trim { output_dir } => trim(&files, &output_dir, sigint, time_off, time_stop),
    };

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
        Ok(_) => println!("Done."),
        Err(xcap::Error::Interrupted) => print!("Interrupted"),
        Err(e) => {
            println!("{}", e);
            print!("Sorry, job failed.");
        }
    }
}
