/* 
 * Management API
 *
 * The management API is as the name states the interface where systems can mutate IAM objects like organizations, projects, clients, users and so on if they have the necessary access rights.
 *
 * OpenAPI spec version: 1.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;
use crate::v2::management::models;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct V1BulkSetOrgMetadataRequest {
  /// The values have to be base64 encoded.
  #[serde(rename = "metadata")]
  metadata: Option<Vec<models::V1BulkSetOrgMetadataRequestMetadata>>
}

impl V1BulkSetOrgMetadataRequest {
  pub fn new() -> V1BulkSetOrgMetadataRequest {
    V1BulkSetOrgMetadataRequest {
      metadata: None
    }
  }

  pub fn set_metadata(&mut self, metadata: Vec<models::V1BulkSetOrgMetadataRequestMetadata>) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: Vec<models::V1BulkSetOrgMetadataRequestMetadata>) -> V1BulkSetOrgMetadataRequest {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&Vec<models::V1BulkSetOrgMetadataRequestMetadata>> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

}



