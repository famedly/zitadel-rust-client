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
pub struct ManagementServiceListProjectGrantMembersBody {
  #[serde(rename = "query")]
  query: Option<models::V1ListQuery>,
  #[serde(rename = "queries")]
  queries: Option<Vec<models::Zitadelmemberv1SearchQuery>>
}

impl ManagementServiceListProjectGrantMembersBody {
  pub fn new() -> ManagementServiceListProjectGrantMembersBody {
    ManagementServiceListProjectGrantMembersBody {
      query: None,
      queries: None
    }
  }

  pub fn set_query(&mut self, query: models::V1ListQuery) {
    self.query = Some(query);
  }

  pub fn with_query(mut self, query: models::V1ListQuery) -> ManagementServiceListProjectGrantMembersBody {
    self.query = Some(query);
    self
  }

  pub fn query(&self) -> Option<&models::V1ListQuery> {
    self.query.as_ref()
  }

  pub fn reset_query(&mut self) {
    self.query = None;
  }

  pub fn set_queries(&mut self, queries: Vec<models::Zitadelmemberv1SearchQuery>) {
    self.queries = Some(queries);
  }

  pub fn with_queries(mut self, queries: Vec<models::Zitadelmemberv1SearchQuery>) -> ManagementServiceListProjectGrantMembersBody {
    self.queries = Some(queries);
    self
  }

  pub fn queries(&self) -> Option<&Vec<models::Zitadelmemberv1SearchQuery>> {
    self.queries.as_ref()
  }

  pub fn reset_queries(&mut self) {
    self.queries = None;
  }

}



