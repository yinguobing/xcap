use clap::Parser;
use env_logger::Env;
use log::{error, info, warn};
use mcap_extractor::storage::Agent;
use rand::Rng;
use std::sync::atomic::AtomicBool;
use std::{env, fs, path::PathBuf, sync::Arc};
use url::Url;

struct RuntimeError(String);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input resorce
    #[arg(short, long)]
    input: String,

    /// Output directory path
    #[arg(short, long)]
    output_dir: Option<PathBuf>,

    /// H264 topic
    #[arg(long)]
    topic: Option<String>,
}

/// Prepare inputs. Download from remote server if necessary.
async fn prepare_inputs(
    args: &Args,
    download_path: &mut Option<PathBuf>,
    sigint: &Arc<AtomicBool>,
) -> Result<Vec<PathBuf>, RuntimeError> {
    // Safety first
    let mut input_src = args.input.clone();
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
        input_src.clone_from(&args.input);
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

#[tokio::main]
async fn main() {
    // Initialization
    let mut download_path: Option<PathBuf> = None;
    let sigint = Arc::new(AtomicBool::new(false));

    // Logger setup
    let log_env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(log_env);

    // Catch SIGINT
    let handler_sigint = sigint.clone();
    ctrlc::set_handler(move || {
        warn!("received Ctrl+C! Mission aborted by user.");
        handler_sigint.store(true, std::sync::atomic::Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    // Parse user args
    let args = Args::parse();

    // Prepare inputs
    let files = match prepare_inputs(&args, &mut download_path, &sigint).await {
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
    let topics = match mcap_extractor::summary(&files) {
        Ok(t) => t,
        Err(e) => {
            error!("{}", e);
            cleanup(&download_path);
            return;
        }
    };
    info!("Found topics: {}", topics.len());
    for topic in topics.iter() {
        info!("- {}", topic);
    }

    // Output directory
    let output_dir = args.output_dir.unwrap_or(std::env::current_dir().unwrap());

    // Process
    info!("Extracting...");
    info!("Output directory: {}", output_dir.display());
    let ret = mcap_extractor::process(&files, &output_dir, &args.topic, sigint);

    // Cleanup
    cleanup(&download_path);

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
