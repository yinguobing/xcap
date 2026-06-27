use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureStringGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureStringGetRequest {
    fn default() -> Self {
        FeatureStringGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl crate::Message for FeatureStringGetRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureStringGetResponse {
    pub value: ::std::string::String,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureStringGetResponse {
    fn default() -> Self {
        FeatureStringGetResponse {
            value: ::std::string::String::new(),
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl crate::Message for FeatureStringGetResponse {}

pub struct FeatureStringGet;
impl crate::Service for FeatureStringGet {
    type Request = FeatureStringGetRequest;
    type Response = FeatureStringGetResponse;

    fn request_type_name(&self) -> &str {
        "FeatureStringGetRequest"
    }
    fn response_type_name(&self) -> &str {
        "FeatureStringGetResponse"
    }
}
