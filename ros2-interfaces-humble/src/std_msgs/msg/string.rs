use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct String {
    pub data: ::std::string::String,
}

impl crate::Message for String {}
