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
pub struct Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
  /// Client id generated by GitHub
  #[serde(rename = "clientId")]
  client_id: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  /// Client secret generated by GitHub
  #[serde(rename = "clientSecret")]
  client_secret: Option<String>,
  #[serde(rename = "authorizationEndpoint")]
  authorization_endpoint: Option<String>,
  #[serde(rename = "tokenEndpoint")]
  token_endpoint: Option<String>,
  #[serde(rename = "userEndpoint")]
  user_endpoint: Option<String>,
  /// The scopes requested by ZITADEL during the request to GitHub
  #[serde(rename = "scopes")]
  scopes: Option<Vec<String>>,
  #[serde(rename = "providerOptions")]
  provider_options: Option<models::V1Options>
}

impl Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
  pub fn new() -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
      client_id: None,
      name: None,
      client_secret: None,
      authorization_endpoint: None,
      token_endpoint: None,
      user_endpoint: None,
      scopes: None,
      provider_options: None
    }
  }

  pub fn set_client_id(&mut self, client_id: String) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: String) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&String> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_client_secret(&mut self, client_secret: String) {
    self.client_secret = Some(client_secret);
  }

  pub fn with_client_secret(mut self, client_secret: String) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    self.client_secret = Some(client_secret);
    self
  }

  pub fn client_secret(&self) -> Option<&String> {
    self.client_secret.as_ref()
  }

  pub fn reset_client_secret(&mut self) {
    self.client_secret = None;
  }

  pub fn set_authorization_endpoint(&mut self, authorization_endpoint: String) {
    self.authorization_endpoint = Some(authorization_endpoint);
  }

  pub fn with_authorization_endpoint(mut self, authorization_endpoint: String) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    self.authorization_endpoint = Some(authorization_endpoint);
    self
  }

  pub fn authorization_endpoint(&self) -> Option<&String> {
    self.authorization_endpoint.as_ref()
  }

  pub fn reset_authorization_endpoint(&mut self) {
    self.authorization_endpoint = None;
  }

  pub fn set_token_endpoint(&mut self, token_endpoint: String) {
    self.token_endpoint = Some(token_endpoint);
  }

  pub fn with_token_endpoint(mut self, token_endpoint: String) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    self.token_endpoint = Some(token_endpoint);
    self
  }

  pub fn token_endpoint(&self) -> Option<&String> {
    self.token_endpoint.as_ref()
  }

  pub fn reset_token_endpoint(&mut self) {
    self.token_endpoint = None;
  }

  pub fn set_user_endpoint(&mut self, user_endpoint: String) {
    self.user_endpoint = Some(user_endpoint);
  }

  pub fn with_user_endpoint(mut self, user_endpoint: String) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    self.user_endpoint = Some(user_endpoint);
    self
  }

  pub fn user_endpoint(&self) -> Option<&String> {
    self.user_endpoint.as_ref()
  }

  pub fn reset_user_endpoint(&mut self) {
    self.user_endpoint = None;
  }

  pub fn set_scopes(&mut self, scopes: Vec<String>) {
    self.scopes = Some(scopes);
  }

  pub fn with_scopes(mut self, scopes: Vec<String>) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
    self.scopes = Some(scopes);
    self
  }

  pub fn scopes(&self) -> Option<&Vec<String>> {
    self.scopes.as_ref()
  }

  pub fn reset_scopes(&mut self) {
    self.scopes = None;
  }

  pub fn set_provider_options(&mut self, provider_options: models::V1Options) {
    self.provider_options = Some(provider_options);
  }

  pub fn with_provider_options(mut self, provider_options: models::V1Options) -> Zitadelmanagementv1AddGitHubEnterpriseServerProviderRequest {
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



