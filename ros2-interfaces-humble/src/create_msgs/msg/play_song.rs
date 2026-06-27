use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaySong {
    pub song: u8,
}

impl Default for PlaySong {
    fn default() -> Self {
        PlaySong { song: 0 }
    }
}

impl crate::Message for PlaySong {}
