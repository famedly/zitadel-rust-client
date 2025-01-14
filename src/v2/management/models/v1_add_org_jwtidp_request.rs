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
pub struct V1AddOrgJwtidpRequest {
  #[serde(rename = "name")]
  name: String,
  /// some identity providers specify the styling of the button to their login
  #[serde(rename = "stylingType")]
  styling_type: Option<models::V1IdpStylingType>,
  /// the endpoint where the JWT can be extracted
  #[serde(rename = "jwtEndpoint")]
  jwt_endpoint: String,
  /// the issuer of the JWT (for validation)
  #[serde(rename = "issuer")]
  issuer: String,
  /// the endpoint to the key (JWK) which is used to sign the JWT with
  #[serde(rename = "keysEndpoint")]
  keys_endpoint: String,
  /// the name of the header where the JWT is sent in, default is authorization
  #[serde(rename = "headerName")]
  header_name: String,
  #[serde(rename = "autoRegister")]
  auto_register: Option<bool>
}

impl V1AddOrgJwtidpRequest {
  pub fn new(name: String, jwt_endpoint: String, issuer: String, keys_endpoint: String, header_name: String) -> V1AddOrgJwtidpRequest {
    V1AddOrgJwtidpRequest {
      name: name,
      styling_type: None,
      jwt_endpoint: jwt_endpoint,
      issuer: issuer,
      keys_endpoint: keys_endpoint,
      header_name: header_name,
      auto_register: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> V1AddOrgJwtidpRequest {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_styling_type(&mut self, styling_type: models::V1IdpStylingType) {
    self.styling_type = Some(styling_type);
  }

  pub fn with_styling_type(mut self, styling_type: models::V1IdpStylingType) -> V1AddOrgJwtidpRequest {
    self.styling_type = Some(styling_type);
    self
  }

  pub fn styling_type(&self) -> Option<&models::V1IdpStylingType> {
    self.styling_type.as_ref()
  }

  pub fn reset_styling_type(&mut self) {
    self.styling_type = None;
  }

  pub fn set_jwt_endpoint(&mut self, jwt_endpoint: String) {
    self.jwt_endpoint = jwt_endpoint;
  }

  pub fn with_jwt_endpoint(mut self, jwt_endpoint: String) -> V1AddOrgJwtidpRequest {
    self.jwt_endpoint = jwt_endpoint;
    self
  }

  pub fn jwt_endpoint(&self) -> &String {
    &self.jwt_endpoint
  }


  pub fn set_issuer(&mut self, issuer: String) {
    self.issuer = issuer;
  }

  pub fn with_issuer(mut self, issuer: String) -> V1AddOrgJwtidpRequest {
    self.issuer = issuer;
    self
  }

  pub fn issuer(&self) -> &String {
    &self.issuer
  }


  pub fn set_keys_endpoint(&mut self, keys_endpoint: String) {
    self.keys_endpoint = keys_endpoint;
  }

  pub fn with_keys_endpoint(mut self, keys_endpoint: String) -> V1AddOrgJwtidpRequest {
    self.keys_endpoint = keys_endpoint;
    self
  }

  pub fn keys_endpoint(&self) -> &String {
    &self.keys_endpoint
  }


  pub fn set_header_name(&mut self, header_name: String) {
    self.header_name = header_name;
  }

  pub fn with_header_name(mut self, header_name: String) -> V1AddOrgJwtidpRequest {
    self.header_name = header_name;
    self
  }

  pub fn header_name(&self) -> &String {
    &self.header_name
  }


  pub fn set_auto_register(&mut self, auto_register: bool) {
    self.auto_register = Some(auto_register);
  }

  pub fn with_auto_register(mut self, auto_register: bool) -> V1AddOrgJwtidpRequest {
    self.auto_register = Some(auto_register);
    self
  }

  pub fn auto_register(&self) -> Option<&bool> {
    self.auto_register.as_ref()
  }

  pub fn reset_auto_register(&mut self) {
    self.auto_register = None;
  }

}



