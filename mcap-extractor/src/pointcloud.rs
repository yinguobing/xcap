use crate::extractor::Extractor;
use mcap::Message;
use ros2_sensor_msgs::msg::PointCloud2;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ZSTD error. {0}")]
    Zstd(#[from] std::io::Error),
    #[error("CDR error. {0}")]
    CDR(#[from] cdr::Error),
    #[error("Unknown error.")]
    Unknown,
}

pub struct Parser {
    output_dir: PathBuf,
}

impl Parser {
    pub fn new(output_path: &Path) -> Self {
        // Create output dir
        fs::create_dir_all(output_path).unwrap();

        Parser {
            output_dir: output_path.into(),
        }
    }
}

impl Extractor for Parser {
    type ExtractorError = Box<dyn std::error::Error>;

    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError> {
        let m: &[u8] = message.data.as_ref();
        let decompressed = zstd::stream::decode_all(m).map_err(|e| Error::Zstd(e))?;
        // for v in decompressed[..8].iter() {
        //     println!("{:x?}", v);
        // }
        // return Err(Box::new(Error::Unknown));

        let raw = cdr::deserialize_from::<_, PointCloud2, _>(
            decompressed.as_slice(),
            cdr::size::Infinite,
        )
        .map_err(|e| Error::CDR(e))?;

        // Create output file
        let mut file = fs::File::create(
            &self
                .output_dir
                .join(format!("{}.bin", message.publish_time)),
        )?;
        file.write_all(&raw.data)?;
        Ok(())
    }

    fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
        Ok(())
    }
}
