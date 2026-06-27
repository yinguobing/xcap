use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetShapeArrayRequest {

}

impl Default for GetShapeArrayRequest {
    fn default() -> Self {
        GetShapeArrayRequest {

        }
    }
}

impl crate::Message for GetShapeArrayRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetShapeArrayResponse {
    pub shapes: crate::tuw_object_msgs::msg::ShapeArray,
}

impl Default for GetShapeArrayResponse {
    fn default() -> Self {
        GetShapeArrayResponse {
            shapes: crate::tuw_object_msgs::msg::ShapeArray::default(),
        }
    }
}

impl crate::Message for GetShapeArrayResponse {}


pub struct GetShapeArray;
impl crate::Service for GetShapeArray {
    type Request = GetShapeArrayRequest;
    type Response = GetShapeArrayResponse;

    fn request_type_name(&self) -> &str { "GetShapeArrayRequest" }
    fn response_type_name(&self) -> &str { "GetShapeArrayResponse" }
}
