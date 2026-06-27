use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramBucket {
    pub bucket_boundary: f64,
    pub count: f64,
}

impl Default for HistogramBucket {
    fn default() -> Self {
        HistogramBucket {
            bucket_boundary: 0.0,
            count: 0.0,
        }
    }
}

impl crate::Message for HistogramBucket {}
