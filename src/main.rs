use clap::Parser;
use mcap::Summary;
use memmap::Mmap;
use ros2_message::dynamic::DynamicMsg;
use std::fs;
use std::path::PathBuf;
mod h264;
mod vehicle;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_summary(data: &[u8]) -> Result<Vec<Option<DynamicMsg>>> {
    let summary = Summary::read(data)?.expect("MCAP files without summary are not supported");

    let (&max_channel_id, _) = summary.channels.iter().max_by_key(|(id, _)| *id).unwrap();
    let max_channel_count = max_channel_id as usize + 1;

    // Message definition for a single channel accesible through:
    // message_definitions[channel_id]
    let mut message_definitions = vec![None; max_channel_count];

    println!(
        "{:<14} File contains {} channels with {} messages",
        "",
        summary.channels.len(),
        summary
            .stats
            .clone()
            .map_or("unknwon".to_owned(), |v| format!("{}", v.message_count))
    );

    // Parse message definitions for all channels
    for (id, channel) in &summary.channels {
        let Some(schema) = &channel.schema else {
            eprintln!("No schema found for channel {}", channel.topic);
            continue;
        };

        if schema.encoding != "ros2msg" {
            println!(
                "Topic {} has unknown encoding {}",
                channel.topic, schema.encoding
            );
            continue; // Ignore channels without ROS2 message
        }

        let msg_name = schema.name.clone();
        let msg_definition_string = String::from_utf8(schema.data.to_vec())?;
        let dynamic_msg = DynamicMsg::new(&msg_name, &msg_definition_string)?;

        // Save message definition
        message_definitions[*id as usize] = Some(dynamic_msg);
    }

    Ok(message_definitions)
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
    let fd = fs::File::open(&args.input).unwrap();
    let buf = unsafe { Mmap::map(&fd) }.unwrap();

    if let Some(summary) = mcap::read::Summary::read(&buf).unwrap() {
        if let Some(stats) = summary.stats {
            println!("Messages: {}", stats.message_count);
        } else {
            println!("Failed to get statistics.")
        }
        println!("Topics:");
        for chn in summary.channels {
            println!("{} {}", chn.0, chn.1.topic);
        }
    } else {
        println!("Failed to read summary info.")
    }

    let data = std::fs::read(args.input).unwrap();
    let message_definitions = parse_summary(&buf).unwrap();
    // for raw_message in mcap::read::RawMessageStream::new(&data).unwrap() {
    //     let raw_message = raw_message.unwrap();
    //     let channel_id = raw_message.header.channel_id as usize;

    //     // Get this channels message defition
    //     let Some(ref dynamic_msg) = message_definitions[channel_id] else {
    //         continue; // Skip channels with unknown schematas
    //     };
    //     if raw_message.header.channel_id != 3 {
    //         continue;
    //     }
    //     let decoded_msg = dynamic_msg.decode_raw(raw_message.data.as_ref());
    //     println!("{} {:#?}", dynamic_msg.msg().path().name(), decoded_msg);
    // }

    // Any extracting jobs?
    if let Some(topic) = args.h264_topic {
        let mut parser = h264::Parser::new(args.output_dir.unwrap(), vehicle::Model::PT1);
        // Gather objects of interests
        let stream = mcap::MessageStream::new(&buf)
            .unwrap()
            .filter(|x| x.as_ref().is_ok_and(|x| x.channel.topic == topic));
        for message in stream {
            let message = message.unwrap();
            parser.process(&message);
        }
    }

    println!("Done.")
}
