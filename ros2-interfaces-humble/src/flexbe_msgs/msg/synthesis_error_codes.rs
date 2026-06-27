use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SynthesisErrorCodes {
    pub value: i32,
}

impl SynthesisErrorCodes {
    pub const SUCCESS: i32 = 1;
}

impl Default for SynthesisErrorCodes {
    fn default() -> Self {
        SynthesisErrorCodes { value: 0 }
    }
}

impl crate::Message for SynthesisErrorCodes {}
