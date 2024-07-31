use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;
mod h264;
mod vehicle;

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

    /// Camera sequence
    #[arg(long)]
    cam_seq: Option<String>,

    /// Verbose mode. This will log MCAP file summary and each processing steps.
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // Summary this file
    println!("File: {:?}", args.input.as_os_str());
    let mut fd = fs::File::open(&args.input).unwrap();
    let mut buf = Vec::new();
    fd.read_to_end(&mut buf).unwrap();
    let Some(summary) = mcap::read::Summary::read(&buf).unwrap() else {
        panic!(
            "Failed to read summary info: {}",
            args.input.to_str().unwrap()
        );
    };

    // Statistics
    if let Some(stats) = summary.stats {
        println!("Messages: {}", stats.message_count);
    } else {
        println!("Failed to get statistics.")
    }

    // Topics
    println!("Topics:");
    let mut channels: Vec<_> = summary.channels.into_iter().collect();
    channels.sort_by_key(|k| k.0);
    for chn in channels {
        println!(" {} {} {:?}", chn.0, chn.1.topic, chn.1.schema);
    }

    // Setup a progress bar.
    let spinner_style = ProgressStyle::default_spinner()
        .template("{prefix:} {spinner} {wide_msg}")
        .unwrap();
    let bar = ProgressBar::new_spinner();
    bar.set_style(spinner_style);
    bar.enable_steady_tick(Duration::from_millis(100));

    // Any extracting jobs?
    if let Some(topic) = args.topic {
        println!("Processing...");
        let mut parser = h264::Parser::new(&args.output_dir.unwrap());
        // Gather objects of interests
        let stream = mcap::MessageStream::new(&buf)
            .unwrap()
            .filter(|x| x.as_ref().is_ok_and(|x| x.channel.topic == topic));
        let mut counter = 0;
        for message in stream {
            let message = message.unwrap();
            parser.process(&message);
            counter += 1;
            bar.set_message(format!("{} {}", topic, counter.to_string()));
        }
        bar.finish();
    } else {
        println!("No topics specified.");
    }
    println!("Done.");
}
