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
pub struct Zitadelorgv1DomainSearchQuery {
  #[serde(rename = "domainNameQuery")]
  domain_name_query: Option<models::V1DomainNameQuery>
}

impl Zitadelorgv1DomainSearchQuery {
  pub fn new() -> Zitadelorgv1DomainSearchQuery {
    Zitadelorgv1DomainSearchQuery {
      domain_name_query: None
    }
  }

  pub fn set_domain_name_query(&mut self, domain_name_query: models::V1DomainNameQuery) {
    self.domain_name_query = Some(domain_name_query);
  }

  pub fn with_domain_name_query(mut self, domain_name_query: models::V1DomainNameQuery) -> Zitadelorgv1DomainSearchQuery {
    self.domain_name_query = Some(domain_name_query);
    self
  }

  pub fn domain_name_query(&self) -> Option<&models::V1DomainNameQuery> {
    self.domain_name_query.as_ref()
  }

  pub fn reset_domain_name_query(&mut self) {
    self.domain_name_query = None;
  }

}



