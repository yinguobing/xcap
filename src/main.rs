use clap::{Parser, ValueEnum};
use memmap::Mmap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum VehicleModel {
    DR1,
    DR2,
    PT1,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// MCAP file to be parsed
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Vehicle model name
    #[arg(short, long)]
    model: Option<VehicleModel>,

    /// H264 topic
    #[arg(long)]
    h264_topic: Option<String>,

    /// Camera sequence
    #[arg(long)]
    cam_seq: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("File: {:?}", args.input.as_os_str());

    let mcap_file_path = args.input;
    let fd = fs::File::open(mcap_file_path).unwrap();
    let buf = unsafe { Mmap::map(&fd) }.unwrap();
    let summary = mcap::read::Summary::read(&buf).unwrap().unwrap();
    println!("Summary\n{:?}", summary.channels.get(&1).unwrap());
    let stream = mcap::MessageStream::new(&buf)
        .unwrap()
        .filter(|x| x.as_ref().is_ok_and(|x| x.channel.topic == "/Data_GPS"));
    for message in stream {
        let m = message.unwrap();
        println!("Message\n{:?}", m);
        break;
    }
}
