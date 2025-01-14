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
pub struct ManagementServiceUpdateSamlAppConfigBody {
  #[serde(rename = "metadataXml")]
  metadata_xml: Option<String>,
  #[serde(rename = "metadataUrl")]
  metadata_url: Option<String>
}

impl ManagementServiceUpdateSamlAppConfigBody {
  pub fn new() -> ManagementServiceUpdateSamlAppConfigBody {
    ManagementServiceUpdateSamlAppConfigBody {
      metadata_xml: None,
      metadata_url: None
    }
  }

  pub fn set_metadata_xml(&mut self, metadata_xml: String) {
    self.metadata_xml = Some(metadata_xml);
  }

  pub fn with_metadata_xml(mut self, metadata_xml: String) -> ManagementServiceUpdateSamlAppConfigBody {
    self.metadata_xml = Some(metadata_xml);
    self
  }

  pub fn metadata_xml(&self) -> Option<&String> {
    self.metadata_xml.as_ref()
  }

  pub fn reset_metadata_xml(&mut self) {
    self.metadata_xml = None;
  }

  pub fn set_metadata_url(&mut self, metadata_url: String) {
    self.metadata_url = Some(metadata_url);
  }

  pub fn with_metadata_url(mut self, metadata_url: String) -> ManagementServiceUpdateSamlAppConfigBody {
    self.metadata_url = Some(metadata_url);
    self
  }

  pub fn metadata_url(&self) -> Option<&String> {
    self.metadata_url.as_ref()
  }

  pub fn reset_metadata_url(&mut self) {
    self.metadata_url = None;
  }

}



