use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxQueryRequest {
    pub min: crate::geometry_msgs::msg::Point,
    pub max: crate::geometry_msgs::msg::Point,
}

impl Default for BoundingBoxQueryRequest {
    fn default() -> Self {
        BoundingBoxQueryRequest {
            min: crate::geometry_msgs::msg::Point::default(),
            max: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl crate::Message for BoundingBoxQueryRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxQueryResponse {}

impl Default for BoundingBoxQueryResponse {
    fn default() -> Self {
        BoundingBoxQueryResponse {}
    }
}

impl crate::Message for BoundingBoxQueryResponse {}

pub struct BoundingBoxQuery;
impl crate::Service for BoundingBoxQuery {
    type Request = BoundingBoxQueryRequest;
    type Response = BoundingBoxQueryResponse;

    fn request_type_name(&self) -> &str {
        "BoundingBoxQueryRequest"
    }
    fn response_type_name(&self) -> &str {
        "BoundingBoxQueryResponse"
    }
}
