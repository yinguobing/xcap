use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationStates {
    pub negotiations: Vec<crate::rmf_traffic_msgs::msg::NegotiationState>,
}

impl Default for NegotiationStates {
    fn default() -> Self {
        NegotiationStates {
            negotiations: Vec::new(),
        }
    }
}

impl crate::Message for NegotiationStates {}
