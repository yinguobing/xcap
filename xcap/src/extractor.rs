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

    /// Wether this extractor produces 3D data
    /// (e.g. point clouds, meshes, etc.)
    fn generates_3d_data(&self) -> bool;

    /// Wether this extractor produces 2D data
    /// (e.g. images, video frames, etc.)
    fn generates_2d_data(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use std::sync::Arc;

    struct TestExtractor;

    impl Extractor for TestExtractor {
        type ExtractorError = Box<dyn std::error::Error>;

        fn step(&mut self, _message: &Message) -> Result<(), Self::ExtractorError> {
            Ok(())
        }
        fn post_process(&mut self, _sigint: Arc<AtomicBool>) -> Result<(), Self::ExtractorError> {
            Ok(())
        }
        fn generates_3d_data(&self) -> bool {
            false
        }
        fn generates_2d_data(&self) -> bool {
            false
        }
    }

    fn make_message(data: Vec<u8>) -> Message<'static> {
        let channel = Arc::new(mcap::Channel {
            id: 1,
            topic: "/test".to_string(),
            schema: None,
            message_encoding: "cdr".to_string(),
            metadata: BTreeMap::new(),
        });
        Message {
            channel,
            sequence: 0,
            log_time: 0,
            publish_time: 0,
            data: Cow::Owned(data),
        }
    }

    #[test]
    fn decode_non_zstd_passthrough() {
        let extractor = TestExtractor;
        let data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let msg = make_message(data.clone());
        let result = extractor.decode(&msg).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn decode_zstd_decompresses() {
        let extractor = TestExtractor;
        let original = b"hello world, this is test data for zstd compression!".to_vec();
        let compressed = zstd::encode_all(original.as_slice(), 0).unwrap();
        let msg = make_message(compressed);
        let result = extractor.decode(&msg).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    #[should_panic(expected = "out of range for slice")]
    fn decode_empty_data() {
        let extractor = TestExtractor;
        let data: Vec<u8> = vec![];
        let msg = make_message(data);
        // Empty data with less than 4 bytes will panic due to index access
        // This documents the current behavior
        let _result = extractor.decode(&msg);
    }

    #[test]
    fn decode_non_zstd_starts_with_zstd_magic_prefix() {
        // If data starts with the ZSTD magic number but is not actually ZSTD-compressed,
        // the decompression should fail.
        let extractor = TestExtractor;
        let mut data = vec![0x28, 0xb5, 0x2f, 0xfd];
        data.extend_from_slice(b"garbage data not actually zstd");
        let msg = make_message(data);
        let result = extractor.decode(&msg);
        assert!(result.is_err());
    }
}
