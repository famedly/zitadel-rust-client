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
pub struct V1IdpLoginPolicyLink {
  /// the id of the identity provider
  #[serde(rename = "idpId")]
  idp_id: Option<String>,
  /// the name of the identity provider
  #[serde(rename = "idpName")]
  idp_name: Option<String>,
  /// the authorization framework of the identity provider
  #[serde(rename = "idpType")]
  idp_type: Option<models::V1IdpType>
}

impl V1IdpLoginPolicyLink {
  pub fn new() -> V1IdpLoginPolicyLink {
    V1IdpLoginPolicyLink {
      idp_id: None,
      idp_name: None,
      idp_type: None
    }
  }

  pub fn set_idp_id(&mut self, idp_id: String) {
    self.idp_id = Some(idp_id);
  }

  pub fn with_idp_id(mut self, idp_id: String) -> V1IdpLoginPolicyLink {
    self.idp_id = Some(idp_id);
    self
  }

  pub fn idp_id(&self) -> Option<&String> {
    self.idp_id.as_ref()
  }

  pub fn reset_idp_id(&mut self) {
    self.idp_id = None;
  }

  pub fn set_idp_name(&mut self, idp_name: String) {
    self.idp_name = Some(idp_name);
  }

  pub fn with_idp_name(mut self, idp_name: String) -> V1IdpLoginPolicyLink {
    self.idp_name = Some(idp_name);
    self
  }

  pub fn idp_name(&self) -> Option<&String> {
    self.idp_name.as_ref()
  }

  pub fn reset_idp_name(&mut self) {
    self.idp_name = None;
  }

  pub fn set_idp_type(&mut self, idp_type: models::V1IdpType) {
    self.idp_type = Some(idp_type);
  }

  pub fn with_idp_type(mut self, idp_type: models::V1IdpType) -> V1IdpLoginPolicyLink {
    self.idp_type = Some(idp_type);
    self
  }

  pub fn idp_type(&self) -> Option<&models::V1IdpType> {
    self.idp_type.as_ref()
  }

  pub fn reset_idp_type(&mut self) {
    self.idp_type = None;
  }

}



