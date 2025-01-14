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
pub struct Zitadelappv1OidcConfig {
  /// Callback URI of the authorization request where the code or tokens will be sent to
  #[serde(rename = "redirectUris")]
  redirect_uris: Option<Vec<String>>,
  /// Determines whether a code, id_token token or just id_token will be returned
  #[serde(rename = "responseTypes")]
  response_types: Option<Vec<models::V1OidcResponseType>>,
  /// The flow type the application uses to gain access
  #[serde(rename = "grantTypes")]
  grant_types: Option<Vec<models::V1OidcGrantType>>,
  /// determines the paradigm of the application
  #[serde(rename = "appType")]
  app_type: Option<models::V1OidcAppType>,
  /// generated oauth2/oidc client id
  #[serde(rename = "clientId")]
  client_id: Option<String>,
  /// defines how the application passes login credentials
  #[serde(rename = "authMethodType")]
  auth_method_type: Option<models::V1OidcAuthMethodType>,
  /// ZITADEL will redirect to this link after a successful logout
  #[serde(rename = "postLogoutRedirectUris")]
  post_logout_redirect_uris: Option<Vec<String>>,
  /// the OIDC version used by the application
  #[serde(rename = "version")]
  version: Option<models::V1OidcVersion>,
  /// specifies whether the config is OIDC compliant. A production configuration SHOULD be compliant
  #[serde(rename = "noneCompliant")]
  none_compliant: Option<bool>,
  /// lists the problems for non-compliancy
  #[serde(rename = "complianceProblems")]
  compliance_problems: Option<Vec<models::V1LocalizedMessage>>,
  /// used for development
  #[serde(rename = "devMode")]
  dev_mode: Option<bool>,
  /// type of the access token returned from ZITADEL
  #[serde(rename = "accessTokenType")]
  access_token_type: Option<models::V1OidcTokenType>,
  /// adds roles to the claims of the access token (only if type == JWT) even if they are not requested by scopes
  #[serde(rename = "accessTokenRoleAssertion")]
  access_token_role_assertion: Option<bool>,
  /// adds roles to the claims of the id token even if they are not requested by scopes
  #[serde(rename = "idTokenRoleAssertion")]
  id_token_role_assertion: Option<bool>,
  /// claims of profile, email, address and phone scopes are added to the id token even if an access token is issued. Attention this violates the OIDC specification
  #[serde(rename = "idTokenUserinfoAssertion")]
  id_token_userinfo_assertion: Option<bool>,
  /// Used to compensate time difference of servers. Duration added to the \"exp\" claim and subtracted from \"iat\", \"auth_time\" and \"nbf\" claims
  #[serde(rename = "clockSkew")]
  clock_skew: Option<String>,
  /// additional origins (other than the redirect_uris) from where the API can be used
  #[serde(rename = "additionalOrigins")]
  additional_origins: Option<Vec<String>>,
  /// all allowed origins from where the API can be used
  #[serde(rename = "allowedOrigins")]
  allowed_origins: Option<Vec<String>>,
  /// Skip the successful login page on native apps and directly redirect the user to the callback.
  #[serde(rename = "skipNativeAppSuccessPage")]
  skip_native_app_success_page: Option<bool>,
  /// ZITADEL will use this URI to notify the application about terminated session according to the OIDC Back-Channel Logout (https://openid.net/specs/openid-connect-backchannel-1_0.html)
  #[serde(rename = "backChannelLogoutUri")]
  back_channel_logout_uri: Option<String>,
  /// Specify the preferred login UI, where the user is redirected to for authentication. If unset, the login UI is chosen by the instance default.
  #[serde(rename = "loginVersion")]
  login_version: Option<models::V1LoginVersion>
}

impl Zitadelappv1OidcConfig {
  pub fn new() -> Zitadelappv1OidcConfig {
    Zitadelappv1OidcConfig {
      redirect_uris: None,
      response_types: None,
      grant_types: None,
      app_type: None,
      client_id: None,
      auth_method_type: None,
      post_logout_redirect_uris: None,
      version: None,
      none_compliant: None,
      compliance_problems: None,
      dev_mode: None,
      access_token_type: None,
      access_token_role_assertion: None,
      id_token_role_assertion: None,
      id_token_userinfo_assertion: None,
      clock_skew: None,
      additional_origins: None,
      allowed_origins: None,
      skip_native_app_success_page: None,
      back_channel_logout_uri: None,
      login_version: None
    }
  }

  pub fn set_redirect_uris(&mut self, redirect_uris: Vec<String>) {
    self.redirect_uris = Some(redirect_uris);
  }

  pub fn with_redirect_uris(mut self, redirect_uris: Vec<String>) -> Zitadelappv1OidcConfig {
    self.redirect_uris = Some(redirect_uris);
    self
  }

  pub fn redirect_uris(&self) -> Option<&Vec<String>> {
    self.redirect_uris.as_ref()
  }

  pub fn reset_redirect_uris(&mut self) {
    self.redirect_uris = None;
  }

  pub fn set_response_types(&mut self, response_types: Vec<models::V1OidcResponseType>) {
    self.response_types = Some(response_types);
  }

  pub fn with_response_types(mut self, response_types: Vec<models::V1OidcResponseType>) -> Zitadelappv1OidcConfig {
    self.response_types = Some(response_types);
    self
  }

  pub fn response_types(&self) -> Option<&Vec<models::V1OidcResponseType>> {
    self.response_types.as_ref()
  }

  pub fn reset_response_types(&mut self) {
    self.response_types = None;
  }

  pub fn set_grant_types(&mut self, grant_types: Vec<models::V1OidcGrantType>) {
    self.grant_types = Some(grant_types);
  }

  pub fn with_grant_types(mut self, grant_types: Vec<models::V1OidcGrantType>) -> Zitadelappv1OidcConfig {
    self.grant_types = Some(grant_types);
    self
  }

  pub fn grant_types(&self) -> Option<&Vec<models::V1OidcGrantType>> {
    self.grant_types.as_ref()
  }

  pub fn reset_grant_types(&mut self) {
    self.grant_types = None;
  }

  pub fn set_app_type(&mut self, app_type: models::V1OidcAppType) {
    self.app_type = Some(app_type);
  }

  pub fn with_app_type(mut self, app_type: models::V1OidcAppType) -> Zitadelappv1OidcConfig {
    self.app_type = Some(app_type);
    self
  }

  pub fn app_type(&self) -> Option<&models::V1OidcAppType> {
    self.app_type.as_ref()
  }

  pub fn reset_app_type(&mut self) {
    self.app_type = None;
  }

  pub fn set_client_id(&mut self, client_id: String) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: String) -> Zitadelappv1OidcConfig {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&String> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_auth_method_type(&mut self, auth_method_type: models::V1OidcAuthMethodType) {
    self.auth_method_type = Some(auth_method_type);
  }

  pub fn with_auth_method_type(mut self, auth_method_type: models::V1OidcAuthMethodType) -> Zitadelappv1OidcConfig {
    self.auth_method_type = Some(auth_method_type);
    self
  }

  pub fn auth_method_type(&self) -> Option<&models::V1OidcAuthMethodType> {
    self.auth_method_type.as_ref()
  }

  pub fn reset_auth_method_type(&mut self) {
    self.auth_method_type = None;
  }

  pub fn set_post_logout_redirect_uris(&mut self, post_logout_redirect_uris: Vec<String>) {
    self.post_logout_redirect_uris = Some(post_logout_redirect_uris);
  }

  pub fn with_post_logout_redirect_uris(mut self, post_logout_redirect_uris: Vec<String>) -> Zitadelappv1OidcConfig {
    self.post_logout_redirect_uris = Some(post_logout_redirect_uris);
    self
  }

  pub fn post_logout_redirect_uris(&self) -> Option<&Vec<String>> {
    self.post_logout_redirect_uris.as_ref()
  }

  pub fn reset_post_logout_redirect_uris(&mut self) {
    self.post_logout_redirect_uris = None;
  }

  pub fn set_version(&mut self, version: models::V1OidcVersion) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: models::V1OidcVersion) -> Zitadelappv1OidcConfig {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&models::V1OidcVersion> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

  pub fn set_none_compliant(&mut self, none_compliant: bool) {
    self.none_compliant = Some(none_compliant);
  }

  pub fn with_none_compliant(mut self, none_compliant: bool) -> Zitadelappv1OidcConfig {
    self.none_compliant = Some(none_compliant);
    self
  }

  pub fn none_compliant(&self) -> Option<&bool> {
    self.none_compliant.as_ref()
  }

  pub fn reset_none_compliant(&mut self) {
    self.none_compliant = None;
  }

  pub fn set_compliance_problems(&mut self, compliance_problems: Vec<models::V1LocalizedMessage>) {
    self.compliance_problems = Some(compliance_problems);
  }

  pub fn with_compliance_problems(mut self, compliance_problems: Vec<models::V1LocalizedMessage>) -> Zitadelappv1OidcConfig {
    self.compliance_problems = Some(compliance_problems);
    self
  }

  pub fn compliance_problems(&self) -> Option<&Vec<models::V1LocalizedMessage>> {
    self.compliance_problems.as_ref()
  }

  pub fn reset_compliance_problems(&mut self) {
    self.compliance_problems = None;
  }

  pub fn set_dev_mode(&mut self, dev_mode: bool) {
    self.dev_mode = Some(dev_mode);
  }

  pub fn with_dev_mode(mut self, dev_mode: bool) -> Zitadelappv1OidcConfig {
    self.dev_mode = Some(dev_mode);
    self
  }

  pub fn dev_mode(&self) -> Option<&bool> {
    self.dev_mode.as_ref()
  }

  pub fn reset_dev_mode(&mut self) {
    self.dev_mode = None;
  }

  pub fn set_access_token_type(&mut self, access_token_type: models::V1OidcTokenType) {
    self.access_token_type = Some(access_token_type);
  }

  pub fn with_access_token_type(mut self, access_token_type: models::V1OidcTokenType) -> Zitadelappv1OidcConfig {
    self.access_token_type = Some(access_token_type);
    self
  }

  pub fn access_token_type(&self) -> Option<&models::V1OidcTokenType> {
    self.access_token_type.as_ref()
  }

  pub fn reset_access_token_type(&mut self) {
    self.access_token_type = None;
  }

  pub fn set_access_token_role_assertion(&mut self, access_token_role_assertion: bool) {
    self.access_token_role_assertion = Some(access_token_role_assertion);
  }

  pub fn with_access_token_role_assertion(mut self, access_token_role_assertion: bool) -> Zitadelappv1OidcConfig {
    self.access_token_role_assertion = Some(access_token_role_assertion);
    self
  }

  pub fn access_token_role_assertion(&self) -> Option<&bool> {
    self.access_token_role_assertion.as_ref()
  }

  pub fn reset_access_token_role_assertion(&mut self) {
    self.access_token_role_assertion = None;
  }

  pub fn set_id_token_role_assertion(&mut self, id_token_role_assertion: bool) {
    self.id_token_role_assertion = Some(id_token_role_assertion);
  }

  pub fn with_id_token_role_assertion(mut self, id_token_role_assertion: bool) -> Zitadelappv1OidcConfig {
    self.id_token_role_assertion = Some(id_token_role_assertion);
    self
  }

  pub fn id_token_role_assertion(&self) -> Option<&bool> {
    self.id_token_role_assertion.as_ref()
  }

  pub fn reset_id_token_role_assertion(&mut self) {
    self.id_token_role_assertion = None;
  }

  pub fn set_id_token_userinfo_assertion(&mut self, id_token_userinfo_assertion: bool) {
    self.id_token_userinfo_assertion = Some(id_token_userinfo_assertion);
  }

  pub fn with_id_token_userinfo_assertion(mut self, id_token_userinfo_assertion: bool) -> Zitadelappv1OidcConfig {
    self.id_token_userinfo_assertion = Some(id_token_userinfo_assertion);
    self
  }

  pub fn id_token_userinfo_assertion(&self) -> Option<&bool> {
    self.id_token_userinfo_assertion.as_ref()
  }

  pub fn reset_id_token_userinfo_assertion(&mut self) {
    self.id_token_userinfo_assertion = None;
  }

  pub fn set_clock_skew(&mut self, clock_skew: String) {
    self.clock_skew = Some(clock_skew);
  }

  pub fn with_clock_skew(mut self, clock_skew: String) -> Zitadelappv1OidcConfig {
    self.clock_skew = Some(clock_skew);
    self
  }

  pub fn clock_skew(&self) -> Option<&String> {
    self.clock_skew.as_ref()
  }

  pub fn reset_clock_skew(&mut self) {
    self.clock_skew = None;
  }

  pub fn set_additional_origins(&mut self, additional_origins: Vec<String>) {
    self.additional_origins = Some(additional_origins);
  }

  pub fn with_additional_origins(mut self, additional_origins: Vec<String>) -> Zitadelappv1OidcConfig {
    self.additional_origins = Some(additional_origins);
    self
  }

  pub fn additional_origins(&self) -> Option<&Vec<String>> {
    self.additional_origins.as_ref()
  }

  pub fn reset_additional_origins(&mut self) {
    self.additional_origins = None;
  }

  pub fn set_allowed_origins(&mut self, allowed_origins: Vec<String>) {
    self.allowed_origins = Some(allowed_origins);
  }

  pub fn with_allowed_origins(mut self, allowed_origins: Vec<String>) -> Zitadelappv1OidcConfig {
    self.allowed_origins = Some(allowed_origins);
    self
  }

  pub fn allowed_origins(&self) -> Option<&Vec<String>> {
    self.allowed_origins.as_ref()
  }

  pub fn reset_allowed_origins(&mut self) {
    self.allowed_origins = None;
  }

  pub fn set_skip_native_app_success_page(&mut self, skip_native_app_success_page: bool) {
    self.skip_native_app_success_page = Some(skip_native_app_success_page);
  }

  pub fn with_skip_native_app_success_page(mut self, skip_native_app_success_page: bool) -> Zitadelappv1OidcConfig {
    self.skip_native_app_success_page = Some(skip_native_app_success_page);
    self
  }

  pub fn skip_native_app_success_page(&self) -> Option<&bool> {
    self.skip_native_app_success_page.as_ref()
  }

  pub fn reset_skip_native_app_success_page(&mut self) {
    self.skip_native_app_success_page = None;
  }

  pub fn set_back_channel_logout_uri(&mut self, back_channel_logout_uri: String) {
    self.back_channel_logout_uri = Some(back_channel_logout_uri);
  }

  pub fn with_back_channel_logout_uri(mut self, back_channel_logout_uri: String) -> Zitadelappv1OidcConfig {
    self.back_channel_logout_uri = Some(back_channel_logout_uri);
    self
  }

  pub fn back_channel_logout_uri(&self) -> Option<&String> {
    self.back_channel_logout_uri.as_ref()
  }

  pub fn reset_back_channel_logout_uri(&mut self) {
    self.back_channel_logout_uri = None;
  }

  pub fn set_login_version(&mut self, login_version: models::V1LoginVersion) {
    self.login_version = Some(login_version);
  }

  pub fn with_login_version(mut self, login_version: models::V1LoginVersion) -> Zitadelappv1OidcConfig {
    self.login_version = Some(login_version);
    self
  }

  pub fn login_version(&self) -> Option<&models::V1LoginVersion> {
    self.login_version.as_ref()
  }

  pub fn reset_login_version(&mut self) {
    self.login_version = None;
  }

}



