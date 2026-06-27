use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValue {
    pub key: ::std::string::String,
    pub value: ::std::string::String,
}

impl crate::Message for KeyValue {}
