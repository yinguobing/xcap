use clap::Parser;
use mcap::Summary;
use ros2_message::dynamic::DynamicMsg;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
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
        println!("- {} {} {:?}", chn.0, chn.1.topic, chn.1.schema);
    }

    // // Any extracting jobs?
    // if let Some(topic) = args.h264_topic {
    //     let mut parser =
    //         h264::Parser::new(&args.output_dir.unwrap(), &args.model.unwrap(), h264_schema);
    //     // Gather objects of interests
    //     let stream = mcap::MessageStream::new(&buf)
    //         .unwrap()
    //         .filter(|x| x.as_ref().is_ok_and(|x| x.channel.topic == topic));
    //     for message in stream {
    //         let message = message.unwrap();
    //         parser.process(&message);
    //     }
    //     // parser.dump_frames();
    // }

    println!("Done.");
}
