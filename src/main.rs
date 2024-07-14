use clap::Parser;
use memmap::Mmap;
use std::fs;
use std::path::PathBuf;

mod h264;
mod vehicle;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// MCAP file to be parsed
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory path
    #[arg(short, long)]
    output_dir: Option<PathBuf>,

    /// Vehicle model name
    #[arg(short, long)]
    model: Option<vehicle::Model>,

    /// H264 topic
    #[arg(long)]
    h264_topic: Option<String>,

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
    let fd = fs::File::open(args.input).unwrap();
    let buf = unsafe { Mmap::map(&fd) }.unwrap();

    if let Some(summary) = mcap::read::Summary::read(&buf).unwrap() {
        if let Some(stats) = summary.stats {
            println!("Messages: {}", stats.message_count);
        } else {
            println!("Failed to get statistics.")
        }
        println!("Topics:");
        for chn in summary.channels {
            println!("- {}", chn.1.topic);
        }
    } else {
        println!("Failed to read summary info.")
    }

    // Any extracting jobs?
    if let Some(topic) = args.h264_topic {
        let mut parser = h264::Parser::new(args.output_dir.unwrap(), vehicle::Model::PT1);
        // Gather objects of interests
        let stream = mcap::MessageStream::new(&buf)
            .unwrap()
            .filter(|x| x.as_ref().is_ok_and(|x| x.channel.topic == topic));
        for message in stream {
            let m = message.unwrap();
            parser.process(&m);
        }
    }

    println!("Done.")
}
