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
pub struct Zitadelmanagementv1IdpQuery {
  #[serde(rename = "idpIdQuery")]
  idp_id_query: Option<models::V1IdpidQuery>,
  #[serde(rename = "idpNameQuery")]
  idp_name_query: Option<models::V1IdpNameQuery>,
  #[serde(rename = "ownerTypeQuery")]
  owner_type_query: Option<models::V1IdpOwnerTypeQuery>
}

impl Zitadelmanagementv1IdpQuery {
  pub fn new() -> Zitadelmanagementv1IdpQuery {
    Zitadelmanagementv1IdpQuery {
      idp_id_query: None,
      idp_name_query: None,
      owner_type_query: None
    }
  }

  pub fn set_idp_id_query(&mut self, idp_id_query: models::V1IdpidQuery) {
    self.idp_id_query = Some(idp_id_query);
  }

  pub fn with_idp_id_query(mut self, idp_id_query: models::V1IdpidQuery) -> Zitadelmanagementv1IdpQuery {
    self.idp_id_query = Some(idp_id_query);
    self
  }

  pub fn idp_id_query(&self) -> Option<&models::V1IdpidQuery> {
    self.idp_id_query.as_ref()
  }

  pub fn reset_idp_id_query(&mut self) {
    self.idp_id_query = None;
  }

  pub fn set_idp_name_query(&mut self, idp_name_query: models::V1IdpNameQuery) {
    self.idp_name_query = Some(idp_name_query);
  }

  pub fn with_idp_name_query(mut self, idp_name_query: models::V1IdpNameQuery) -> Zitadelmanagementv1IdpQuery {
    self.idp_name_query = Some(idp_name_query);
    self
  }

  pub fn idp_name_query(&self) -> Option<&models::V1IdpNameQuery> {
    self.idp_name_query.as_ref()
  }

  pub fn reset_idp_name_query(&mut self) {
    self.idp_name_query = None;
  }

  pub fn set_owner_type_query(&mut self, owner_type_query: models::V1IdpOwnerTypeQuery) {
    self.owner_type_query = Some(owner_type_query);
  }

  pub fn with_owner_type_query(mut self, owner_type_query: models::V1IdpOwnerTypeQuery) -> Zitadelmanagementv1IdpQuery {
    self.owner_type_query = Some(owner_type_query);
    self
  }

  pub fn owner_type_query(&self) -> Option<&models::V1IdpOwnerTypeQuery> {
    self.owner_type_query.as_ref()
  }

  pub fn reset_owner_type_query(&mut self) {
    self.owner_type_query = None;
  }

}



