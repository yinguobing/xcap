use mcap::Message;
use std::sync::{atomic::AtomicBool, Arc};
const ZSTD_MAGIC_NUMBER: [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];

pub trait Extractor {
    type ExtractorError;

    /// Decode the compressed message
    fn decode(&self, message: &Message) -> Result<Vec<u8>, std::io::Error> {
        if message.data[..4] == ZSTD_MAGIC_NUMBER {
            zstd::stream::decode_all(message.data.as_ref())
        } else {
            Ok(message.data.to_vec())
        }
    }

    /// Function to be called for every message.
    fn step(&mut self, message: &Message) -> Result<(), Self::ExtractorError>;

    /// Function to be called after all messages have been processed.
    fn post_process(&mut self, sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError>;
}
