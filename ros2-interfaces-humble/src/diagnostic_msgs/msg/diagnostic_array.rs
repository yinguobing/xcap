use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticArray {
    pub header: crate::std_msgs::msg::Header,
    pub status: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
}

impl crate::Message for DiagnosticArray {}
