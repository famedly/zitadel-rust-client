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
pub struct V1ManagementServiceUpdateJwtProviderBody {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "issuer")]
  issuer: Option<String>,
  #[serde(rename = "jwtEndpoint")]
  jwt_endpoint: Option<String>,
  #[serde(rename = "keysEndpoint")]
  keys_endpoint: Option<String>,
  #[serde(rename = "headerName")]
  header_name: Option<String>,
  #[serde(rename = "providerOptions")]
  provider_options: Option<models::V1Options>
}

impl V1ManagementServiceUpdateJwtProviderBody {
  pub fn new() -> V1ManagementServiceUpdateJwtProviderBody {
    V1ManagementServiceUpdateJwtProviderBody {
      name: None,
      issuer: None,
      jwt_endpoint: None,
      keys_endpoint: None,
      header_name: None,
      provider_options: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> V1ManagementServiceUpdateJwtProviderBody {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_issuer(&mut self, issuer: String) {
    self.issuer = Some(issuer);
  }

  pub fn with_issuer(mut self, issuer: String) -> V1ManagementServiceUpdateJwtProviderBody {
    self.issuer = Some(issuer);
    self
  }

  pub fn issuer(&self) -> Option<&String> {
    self.issuer.as_ref()
  }

  pub fn reset_issuer(&mut self) {
    self.issuer = None;
  }

  pub fn set_jwt_endpoint(&mut self, jwt_endpoint: String) {
    self.jwt_endpoint = Some(jwt_endpoint);
  }

  pub fn with_jwt_endpoint(mut self, jwt_endpoint: String) -> V1ManagementServiceUpdateJwtProviderBody {
    self.jwt_endpoint = Some(jwt_endpoint);
    self
  }

  pub fn jwt_endpoint(&self) -> Option<&String> {
    self.jwt_endpoint.as_ref()
  }

  pub fn reset_jwt_endpoint(&mut self) {
    self.jwt_endpoint = None;
  }

  pub fn set_keys_endpoint(&mut self, keys_endpoint: String) {
    self.keys_endpoint = Some(keys_endpoint);
  }

  pub fn with_keys_endpoint(mut self, keys_endpoint: String) -> V1ManagementServiceUpdateJwtProviderBody {
    self.keys_endpoint = Some(keys_endpoint);
    self
  }

  pub fn keys_endpoint(&self) -> Option<&String> {
    self.keys_endpoint.as_ref()
  }

  pub fn reset_keys_endpoint(&mut self) {
    self.keys_endpoint = None;
  }

  pub fn set_header_name(&mut self, header_name: String) {
    self.header_name = Some(header_name);
  }

  pub fn with_header_name(mut self, header_name: String) -> V1ManagementServiceUpdateJwtProviderBody {
    self.header_name = Some(header_name);
    self
  }

  pub fn header_name(&self) -> Option<&String> {
    self.header_name.as_ref()
  }

  pub fn reset_header_name(&mut self) {
    self.header_name = None;
  }

  pub fn set_provider_options(&mut self, provider_options: models::V1Options) {
    self.provider_options = Some(provider_options);
  }

  pub fn with_provider_options(mut self, provider_options: models::V1Options) -> V1ManagementServiceUpdateJwtProviderBody {
    self.provider_options = Some(provider_options);
    self
  }

  pub fn provider_options(&self) -> Option<&models::V1Options> {
    self.provider_options.as_ref()
  }

  pub fn reset_provider_options(&mut self) {
    self.provider_options = None;
  }

}



