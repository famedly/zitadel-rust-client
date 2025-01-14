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
pub struct ManagementServiceUpdateProjectRoleBody {
  #[serde(rename = "displayName")]
  display_name: String,
  /// The group is only used for display purposes. That you have better handling, like giving all the roles from a group to a user.
  #[serde(rename = "group")]
  group: Option<String>
}

impl ManagementServiceUpdateProjectRoleBody {
  pub fn new(display_name: String) -> ManagementServiceUpdateProjectRoleBody {
    ManagementServiceUpdateProjectRoleBody {
      display_name: display_name,
      group: None
    }
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = display_name;
  }

  pub fn with_display_name(mut self, display_name: String) -> ManagementServiceUpdateProjectRoleBody {
    self.display_name = display_name;
    self
  }

  pub fn display_name(&self) -> &String {
    &self.display_name
  }


  pub fn set_group(&mut self, group: String) {
    self.group = Some(group);
  }

  pub fn with_group(mut self, group: String) -> ManagementServiceUpdateProjectRoleBody {
    self.group = Some(group);
    self
  }

  pub fn group(&self) -> Option<&String> {
    self.group.as_ref()
  }

  pub fn reset_group(&mut self) {
    self.group = None;
  }

}



