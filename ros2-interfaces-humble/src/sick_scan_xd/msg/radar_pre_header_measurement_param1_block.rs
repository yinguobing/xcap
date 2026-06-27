use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarPreHeaderMeasurementParam1Block {
    pub uicycleduration: u32,
    pub uinoiselevel: u32,
}

impl Default for RadarPreHeaderMeasurementParam1Block {
    fn default() -> Self {
        RadarPreHeaderMeasurementParam1Block {
            uicycleduration: 0,
            uinoiselevel: 0,
        }
    }
}

impl crate::Message for RadarPreHeaderMeasurementParam1Block {}
