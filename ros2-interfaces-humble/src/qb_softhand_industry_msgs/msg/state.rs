use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub actuator: crate::qb_softhand_industry_msgs::msg::ResourceData,
}

impl Default for State {
    fn default() -> Self {
        State {
            actuator: crate::qb_softhand_industry_msgs::msg::ResourceData::default(),
        }
    }
}

impl crate::Message for State {}
