use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturesListGetRequest {
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeaturesListGetRequest {
    fn default() -> Self {
        FeaturesListGetRequest {
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl crate::Message for FeaturesListGetRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturesListGetResponse {
    pub feature_list: Vec<::std::string::String>,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeaturesListGetResponse {
    fn default() -> Self {
        FeaturesListGetResponse {
            feature_list: Vec::new(),
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl crate::Message for FeaturesListGetResponse {}

pub struct FeaturesListGet;
impl crate::Service for FeaturesListGet {
    type Request = FeaturesListGetRequest;
    type Response = FeaturesListGetResponse;

    fn request_type_name(&self) -> &str {
        "FeaturesListGetRequest"
    }
    fn response_type_name(&self) -> &str {
        "FeaturesListGetResponse"
    }
}
