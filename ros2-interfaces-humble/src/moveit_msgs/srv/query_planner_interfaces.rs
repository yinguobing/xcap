use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesRequest {}

impl Default for QueryPlannerInterfacesRequest {
    fn default() -> Self {
        QueryPlannerInterfacesRequest {}
    }
}

impl crate::Message for QueryPlannerInterfacesRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesResponse {
    pub planner_interfaces: Vec<crate::moveit_msgs::msg::PlannerInterfaceDescription>,
}

impl Default for QueryPlannerInterfacesResponse {
    fn default() -> Self {
        QueryPlannerInterfacesResponse {
            planner_interfaces: Vec::new(),
        }
    }
}

impl crate::Message for QueryPlannerInterfacesResponse {}

pub struct QueryPlannerInterfaces;
impl crate::Service for QueryPlannerInterfaces {
    type Request = QueryPlannerInterfacesRequest;
    type Response = QueryPlannerInterfacesResponse;

    fn request_type_name(&self) -> &str {
        "QueryPlannerInterfacesRequest"
    }
    fn response_type_name(&self) -> &str {
        "QueryPlannerInterfacesResponse"
    }
}
