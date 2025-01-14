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
pub struct Orgv1Org {
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "details")]
  details: Option<models::V1ObjectDetails>,
  /// current state of the organization
  #[serde(rename = "state")]
  state: Option<models::V1OrgState>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "primaryDomain")]
  primary_domain: Option<String>
}

impl Orgv1Org {
  pub fn new() -> Orgv1Org {
    Orgv1Org {
      id: None,
      details: None,
      state: None,
      name: None,
      primary_domain: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> Orgv1Org {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_details(&mut self, details: models::V1ObjectDetails) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: models::V1ObjectDetails) -> Orgv1Org {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&models::V1ObjectDetails> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

  pub fn set_state(&mut self, state: models::V1OrgState) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: models::V1OrgState) -> Orgv1Org {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&models::V1OrgState> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Orgv1Org {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_primary_domain(&mut self, primary_domain: String) {
    self.primary_domain = Some(primary_domain);
  }

  pub fn with_primary_domain(mut self, primary_domain: String) -> Orgv1Org {
    self.primary_domain = Some(primary_domain);
    self
  }

  pub fn primary_domain(&self) -> Option<&String> {
    self.primary_domain.as_ref()
  }

  pub fn reset_primary_domain(&mut self) {
    self.primary_domain = None;
  }

}



