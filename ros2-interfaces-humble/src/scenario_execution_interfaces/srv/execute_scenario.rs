use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScenarioRequest {
    pub scenario: crate::scenario_execution_interfaces::msg::Scenario,
}

impl Default for ExecuteScenarioRequest {
    fn default() -> Self {
        ExecuteScenarioRequest {
            scenario: crate::scenario_execution_interfaces::msg::Scenario::default(),
        }
    }
}

impl crate::Message for ExecuteScenarioRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScenarioResponse {
    pub result: bool,
}

impl Default for ExecuteScenarioResponse {
    fn default() -> Self {
        ExecuteScenarioResponse { result: false }
    }
}

impl crate::Message for ExecuteScenarioResponse {}

pub struct ExecuteScenario;
impl crate::Service for ExecuteScenario {
    type Request = ExecuteScenarioRequest;
    type Response = ExecuteScenarioResponse;

    fn request_type_name(&self) -> &str {
        "ExecuteScenarioRequest"
    }
    fn response_type_name(&self) -> &str {
        "ExecuteScenarioResponse"
    }
}
