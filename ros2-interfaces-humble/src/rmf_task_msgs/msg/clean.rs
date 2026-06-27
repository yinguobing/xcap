use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clean {
    pub start_waypoint: ::std::string::String,
}

impl Default for Clean {
    fn default() -> Self {
        Clean {
            start_waypoint: ::std::string::String::new(),
        }
    }
}

impl crate::Message for Clean {}
