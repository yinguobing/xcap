use clap::Parser;
use env_logger::Env;
use log::error;
use mcap_extractor::storage;
use std::{fs, path::PathBuf};
use tokio;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// MCAP file to be parsed
    #[arg(short, long)]
    input: PathBuf,

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
    let env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    // Download from remote server?
    let _ = storage::download("base_url", "bucket", "path").await;

    // Safety first
    if !args.input.exists() {
        error!("Input directory not found: {:?}", args.input.as_os_str());
        return;
    }

    // Find all MCAP files
    let mut files: Vec<PathBuf> = fs::read_dir(&args.input)
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.is_file() && f.extension().is_some_and(|f| f.eq("mcap")))
        .collect();
    if files.is_empty() {
        error!("No MCAP files found in path: {}", args.input.display());
        return;
    }
    files.sort();
    println!("Found MCAP files: {}", files.len());
    for f in files.iter() {
        println!("- {}", f.display());
    }

    // Output directory
    let output_dir = args.output_dir.unwrap_or(std::env::current_dir().unwrap());

    // Process
    match mcap_extractor::process(&files, &output_dir, &args.topic) {
        Ok(_) => {
            println!("Done.");
        }
        Err(e) => {
            error!("{}", e);
            println!("Sorry, job failed.");
        }
    }
}
