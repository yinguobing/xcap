use mcap::Message;
use std::sync::{atomic::AtomicBool, Arc};

pub trait Extractor {
    type Error;

    /// Function to be called for every message.
    fn step(&mut self, message: &Message) -> Result<(), Self::Error>;

    /// Function to be called after all messages have been processed.
    fn post_process(&mut self, sigint: Arc<AtomicBool>) -> Result<(), Self::Error>;
}
