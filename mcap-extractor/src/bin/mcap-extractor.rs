use clap::Parser;
use env_logger::Env;
use log::{error, info};
use mcap_extractor::storage::Agent;
use rand::Rng;
use std::{env, fs, path::PathBuf};
use url::Url;

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

#[tokio::main]
async fn main() {
    // Logger setup
    let log_env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(log_env);

    // Parse user args
    let args = Args::parse();

    // Safety first
    let mut input_src = args.input.clone();
    if input_src.is_empty() {
        error!("Input source is empty.");
        return;
    }

    // Download from remote server?
    let mut download_path: Option<PathBuf> = None;
    if input_src.starts_with("http") {
        let valid_url = match Url::parse(&input_src) {
            Ok(url) => url,
            Err(e) => {
                error!("Invalid URL: {}", e);
                return;
            }
        };

        let base_url = format!(
            "{}://{}:{}",
            valid_url.scheme(),
            valid_url.host_str().expect("Invalid url host"),
            valid_url.port().expect("Invalid url port")
        );
        let bucket = valid_url
            .path_segments()
            .expect("Invalid url, valid path expected.")
            .next()
            .expect("Invalid url path, bucket name expected.");
        let obj_name = valid_url
            .path_segments()
            .unwrap()
            .last()
            .expect("Invalid url path, object name expected.");
        let object_dir = valid_url
            .path()
            .trim_start_matches('/')
            .trim_start_matches(bucket)
            .trim_start_matches('/')
            .trim_end_matches(obj_name)
            .trim_end_matches('/');

        let Ok(region) = env::var("S3_REGION") else {
            error!("Environment variable `S3_REGION` not set.");
            return;
        };
        let Ok(access_key) = env::var("S3_ACCESS_KEY") else {
            error!("Environment variable `S3_ACCESS_KEY` not set.");
            return;
        };
        let Ok(secret_key) = env::var("S3_SECRET_KEY") else {
            error!("Environment variable `S3_SECRET_KEY` not set.");
            return;
        };

        let storage = match Agent::new(&base_url, &region, &access_key, &secret_key) {
            Ok(agent) => agent,
            Err(e) => {
                error!("Storage init failed. {}", e);
                return;
            }
        };

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
        download_path = Some(_down_path.clone());
        match std::fs::create_dir_all(&_down_path) {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "Failed to create download directory: {}, {}",
                    _down_path.display(),
                    e
                );
                return;
            }
        }

        info!("Downloading objects from: {}", object_dir);
        match storage.download_dir(bucket, object_dir, &_down_path).await {
            Ok(_) => {
                info!("Download succeeded.");
            }
            Err(e) => {
                error!("Download failed. {}", e);
                return;
            }
        }
        input_src = _down_path.into_os_string().into_string().unwrap();
    } else {
        input_src.clone_from(&args.input);
    }

    let input_dir = PathBuf::from(input_src);
    if !input_dir.exists() {
        error!("Input directory not found: {}", input_dir.display());
        return;
    }

    // Find all MCAP files
    let mut files: Vec<PathBuf> = fs::read_dir(&input_dir)
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.is_file() && f.extension().is_some_and(|f| f.eq("mcap")))
        .collect();
    if files.is_empty() {
        error!("No MCAP files found in path: {}", input_dir.display());
        return;
    }
    files.sort();
    info!("Found MCAP files: {}", files.len());
    for f in files.iter() {
        info!("- {}", f.display());
    }

    // Output directory
    let output_dir = args.output_dir.unwrap_or(std::env::current_dir().unwrap());

    // Process
    match mcap_extractor::process(&files, &output_dir, &args.topic) {
        Ok(_) => {
            info!("Done.");
        }
        Err(e) => {
            error!("{}", e);
            info!("Sorry, job failed.");
        }
    }

    // Cleanup
    if let Some(path) = download_path {
        match std::fs::remove_dir_all(path) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to remove download directory: {}", e);
            }
        }
    }
}
