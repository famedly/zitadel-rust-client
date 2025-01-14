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
pub struct ManagementServiceSetTriggerActionsBody {
  #[serde(rename = "actionIds")]
  action_ids: Option<Vec<String>>
}

impl ManagementServiceSetTriggerActionsBody {
  pub fn new() -> ManagementServiceSetTriggerActionsBody {
    ManagementServiceSetTriggerActionsBody {
      action_ids: None
    }
  }

  pub fn set_action_ids(&mut self, action_ids: Vec<String>) {
    self.action_ids = Some(action_ids);
  }

  pub fn with_action_ids(mut self, action_ids: Vec<String>) -> ManagementServiceSetTriggerActionsBody {
    self.action_ids = Some(action_ids);
    self
  }

  pub fn action_ids(&self) -> Option<&Vec<String>> {
    self.action_ids.as_ref()
  }

  pub fn reset_action_ids(&mut self) {
    self.action_ids = None;
  }

}



