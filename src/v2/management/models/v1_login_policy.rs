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
pub struct V1LoginPolicy {
  #[serde(rename = "details")]
  details: Option<models::V1ObjectDetails>,
  /// defines if a user is allowed to log in with username and password
  #[serde(rename = "allowUsernamePassword")]
  allow_username_password: Option<bool>,
  /// defines if a person is allowed to register a user on this organization
  #[serde(rename = "allowRegister")]
  allow_register: Option<bool>,
  /// defines if a user is allowed to add a defined identity provider. E.g. Google auth
  #[serde(rename = "allowExternalIdp")]
  allow_external_idp: Option<bool>,
  /// defines if a user MUST use a multi-factor to log in
  #[serde(rename = "forceMfa")]
  force_mfa: Option<bool>,
  /// defines if passwordless is allowed for users
  #[serde(rename = "passwordlessType")]
  passwordless_type: Option<models::V1PasswordlessType>,
  /// defines if the organization's admin changed the policy
  #[serde(rename = "isDefault")]
  is_default: Option<bool>,
  /// defines if password reset link should be shown in the login screen
  #[serde(rename = "hidePasswordReset")]
  hide_password_reset: Option<bool>,
  /// defines if unknown username on login screen directly returns an error or always displays the password screen
  #[serde(rename = "ignoreUnknownUsernames")]
  ignore_unknown_usernames: Option<bool>,
  /// defines where the user will be redirected to if the login is started without app context (e.g. from mail)
  #[serde(rename = "defaultRedirectUri")]
  default_redirect_uri: Option<String>,
  #[serde(rename = "passwordCheckLifetime")]
  password_check_lifetime: Option<String>,
  #[serde(rename = "externalLoginCheckLifetime")]
  external_login_check_lifetime: Option<String>,
  #[serde(rename = "mfaInitSkipLifetime")]
  mfa_init_skip_lifetime: Option<String>,
  #[serde(rename = "secondFactorCheckLifetime")]
  second_factor_check_lifetime: Option<String>,
  #[serde(rename = "multiFactorCheckLifetime")]
  multi_factor_check_lifetime: Option<String>,
  #[serde(rename = "secondFactors")]
  second_factors: Option<Vec<models::V1SecondFactorType>>,
  #[serde(rename = "multiFactors")]
  multi_factors: Option<Vec<models::V1MultiFactorType>>,
  #[serde(rename = "idps")]
  idps: Option<Vec<models::V1IdpLoginPolicyLink>>,
  /// If set to true, the suffix (@domain.com) of an unknown username input on the login screen will be matched against the org domains and will redirect to the registration of that organization on success.
  #[serde(rename = "allowDomainDiscovery")]
  allow_domain_discovery: Option<bool>,
  /// defines if the user can additionally (to the login name) be identified by their verified email address
  #[serde(rename = "disableLoginWithEmail")]
  disable_login_with_email: Option<bool>,
  /// defines if the user can additionally (to the login name) be identified by their verified phone number
  #[serde(rename = "disableLoginWithPhone")]
  disable_login_with_phone: Option<bool>,
  /// if activated, only local authenticated users are forced to use MFA. Authentication through IDPs won't prompt a MFA step in the login.
  #[serde(rename = "forceMfaLocalOnly")]
  force_mfa_local_only: Option<bool>
}

impl V1LoginPolicy {
  pub fn new() -> V1LoginPolicy {
    V1LoginPolicy {
      details: None,
      allow_username_password: None,
      allow_register: None,
      allow_external_idp: None,
      force_mfa: None,
      passwordless_type: None,
      is_default: None,
      hide_password_reset: None,
      ignore_unknown_usernames: None,
      default_redirect_uri: None,
      password_check_lifetime: None,
      external_login_check_lifetime: None,
      mfa_init_skip_lifetime: None,
      second_factor_check_lifetime: None,
      multi_factor_check_lifetime: None,
      second_factors: None,
      multi_factors: None,
      idps: None,
      allow_domain_discovery: None,
      disable_login_with_email: None,
      disable_login_with_phone: None,
      force_mfa_local_only: None
    }
  }

  pub fn set_details(&mut self, details: models::V1ObjectDetails) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: models::V1ObjectDetails) -> V1LoginPolicy {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&models::V1ObjectDetails> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

  pub fn set_allow_username_password(&mut self, allow_username_password: bool) {
    self.allow_username_password = Some(allow_username_password);
  }

  pub fn with_allow_username_password(mut self, allow_username_password: bool) -> V1LoginPolicy {
    self.allow_username_password = Some(allow_username_password);
    self
  }

  pub fn allow_username_password(&self) -> Option<&bool> {
    self.allow_username_password.as_ref()
  }

  pub fn reset_allow_username_password(&mut self) {
    self.allow_username_password = None;
  }

  pub fn set_allow_register(&mut self, allow_register: bool) {
    self.allow_register = Some(allow_register);
  }

  pub fn with_allow_register(mut self, allow_register: bool) -> V1LoginPolicy {
    self.allow_register = Some(allow_register);
    self
  }

  pub fn allow_register(&self) -> Option<&bool> {
    self.allow_register.as_ref()
  }

  pub fn reset_allow_register(&mut self) {
    self.allow_register = None;
  }

  pub fn set_allow_external_idp(&mut self, allow_external_idp: bool) {
    self.allow_external_idp = Some(allow_external_idp);
  }

  pub fn with_allow_external_idp(mut self, allow_external_idp: bool) -> V1LoginPolicy {
    self.allow_external_idp = Some(allow_external_idp);
    self
  }

  pub fn allow_external_idp(&self) -> Option<&bool> {
    self.allow_external_idp.as_ref()
  }

  pub fn reset_allow_external_idp(&mut self) {
    self.allow_external_idp = None;
  }

  pub fn set_force_mfa(&mut self, force_mfa: bool) {
    self.force_mfa = Some(force_mfa);
  }

  pub fn with_force_mfa(mut self, force_mfa: bool) -> V1LoginPolicy {
    self.force_mfa = Some(force_mfa);
    self
  }

  pub fn force_mfa(&self) -> Option<&bool> {
    self.force_mfa.as_ref()
  }

  pub fn reset_force_mfa(&mut self) {
    self.force_mfa = None;
  }

  pub fn set_passwordless_type(&mut self, passwordless_type: models::V1PasswordlessType) {
    self.passwordless_type = Some(passwordless_type);
  }

  pub fn with_passwordless_type(mut self, passwordless_type: models::V1PasswordlessType) -> V1LoginPolicy {
    self.passwordless_type = Some(passwordless_type);
    self
  }

  pub fn passwordless_type(&self) -> Option<&models::V1PasswordlessType> {
    self.passwordless_type.as_ref()
  }

  pub fn reset_passwordless_type(&mut self) {
    self.passwordless_type = None;
  }

  pub fn set_is_default(&mut self, is_default: bool) {
    self.is_default = Some(is_default);
  }

  pub fn with_is_default(mut self, is_default: bool) -> V1LoginPolicy {
    self.is_default = Some(is_default);
    self
  }

  pub fn is_default(&self) -> Option<&bool> {
    self.is_default.as_ref()
  }

  pub fn reset_is_default(&mut self) {
    self.is_default = None;
  }

  pub fn set_hide_password_reset(&mut self, hide_password_reset: bool) {
    self.hide_password_reset = Some(hide_password_reset);
  }

  pub fn with_hide_password_reset(mut self, hide_password_reset: bool) -> V1LoginPolicy {
    self.hide_password_reset = Some(hide_password_reset);
    self
  }

  pub fn hide_password_reset(&self) -> Option<&bool> {
    self.hide_password_reset.as_ref()
  }

  pub fn reset_hide_password_reset(&mut self) {
    self.hide_password_reset = None;
  }

  pub fn set_ignore_unknown_usernames(&mut self, ignore_unknown_usernames: bool) {
    self.ignore_unknown_usernames = Some(ignore_unknown_usernames);
  }

  pub fn with_ignore_unknown_usernames(mut self, ignore_unknown_usernames: bool) -> V1LoginPolicy {
    self.ignore_unknown_usernames = Some(ignore_unknown_usernames);
    self
  }

  pub fn ignore_unknown_usernames(&self) -> Option<&bool> {
    self.ignore_unknown_usernames.as_ref()
  }

  pub fn reset_ignore_unknown_usernames(&mut self) {
    self.ignore_unknown_usernames = None;
  }

  pub fn set_default_redirect_uri(&mut self, default_redirect_uri: String) {
    self.default_redirect_uri = Some(default_redirect_uri);
  }

  pub fn with_default_redirect_uri(mut self, default_redirect_uri: String) -> V1LoginPolicy {
    self.default_redirect_uri = Some(default_redirect_uri);
    self
  }

  pub fn default_redirect_uri(&self) -> Option<&String> {
    self.default_redirect_uri.as_ref()
  }

  pub fn reset_default_redirect_uri(&mut self) {
    self.default_redirect_uri = None;
  }

  pub fn set_password_check_lifetime(&mut self, password_check_lifetime: String) {
    self.password_check_lifetime = Some(password_check_lifetime);
  }

  pub fn with_password_check_lifetime(mut self, password_check_lifetime: String) -> V1LoginPolicy {
    self.password_check_lifetime = Some(password_check_lifetime);
    self
  }

  pub fn password_check_lifetime(&self) -> Option<&String> {
    self.password_check_lifetime.as_ref()
  }

  pub fn reset_password_check_lifetime(&mut self) {
    self.password_check_lifetime = None;
  }

  pub fn set_external_login_check_lifetime(&mut self, external_login_check_lifetime: String) {
    self.external_login_check_lifetime = Some(external_login_check_lifetime);
  }

  pub fn with_external_login_check_lifetime(mut self, external_login_check_lifetime: String) -> V1LoginPolicy {
    self.external_login_check_lifetime = Some(external_login_check_lifetime);
    self
  }

  pub fn external_login_check_lifetime(&self) -> Option<&String> {
    self.external_login_check_lifetime.as_ref()
  }

  pub fn reset_external_login_check_lifetime(&mut self) {
    self.external_login_check_lifetime = None;
  }

  pub fn set_mfa_init_skip_lifetime(&mut self, mfa_init_skip_lifetime: String) {
    self.mfa_init_skip_lifetime = Some(mfa_init_skip_lifetime);
  }

  pub fn with_mfa_init_skip_lifetime(mut self, mfa_init_skip_lifetime: String) -> V1LoginPolicy {
    self.mfa_init_skip_lifetime = Some(mfa_init_skip_lifetime);
    self
  }

  pub fn mfa_init_skip_lifetime(&self) -> Option<&String> {
    self.mfa_init_skip_lifetime.as_ref()
  }

  pub fn reset_mfa_init_skip_lifetime(&mut self) {
    self.mfa_init_skip_lifetime = None;
  }

  pub fn set_second_factor_check_lifetime(&mut self, second_factor_check_lifetime: String) {
    self.second_factor_check_lifetime = Some(second_factor_check_lifetime);
  }

  pub fn with_second_factor_check_lifetime(mut self, second_factor_check_lifetime: String) -> V1LoginPolicy {
    self.second_factor_check_lifetime = Some(second_factor_check_lifetime);
    self
  }

  pub fn second_factor_check_lifetime(&self) -> Option<&String> {
    self.second_factor_check_lifetime.as_ref()
  }

  pub fn reset_second_factor_check_lifetime(&mut self) {
    self.second_factor_check_lifetime = None;
  }

  pub fn set_multi_factor_check_lifetime(&mut self, multi_factor_check_lifetime: String) {
    self.multi_factor_check_lifetime = Some(multi_factor_check_lifetime);
  }

  pub fn with_multi_factor_check_lifetime(mut self, multi_factor_check_lifetime: String) -> V1LoginPolicy {
    self.multi_factor_check_lifetime = Some(multi_factor_check_lifetime);
    self
  }

  pub fn multi_factor_check_lifetime(&self) -> Option<&String> {
    self.multi_factor_check_lifetime.as_ref()
  }

  pub fn reset_multi_factor_check_lifetime(&mut self) {
    self.multi_factor_check_lifetime = None;
  }

  pub fn set_second_factors(&mut self, second_factors: Vec<models::V1SecondFactorType>) {
    self.second_factors = Some(second_factors);
  }

  pub fn with_second_factors(mut self, second_factors: Vec<models::V1SecondFactorType>) -> V1LoginPolicy {
    self.second_factors = Some(second_factors);
    self
  }

  pub fn second_factors(&self) -> Option<&Vec<models::V1SecondFactorType>> {
    self.second_factors.as_ref()
  }

  pub fn reset_second_factors(&mut self) {
    self.second_factors = None;
  }

  pub fn set_multi_factors(&mut self, multi_factors: Vec<models::V1MultiFactorType>) {
    self.multi_factors = Some(multi_factors);
  }

  pub fn with_multi_factors(mut self, multi_factors: Vec<models::V1MultiFactorType>) -> V1LoginPolicy {
    self.multi_factors = Some(multi_factors);
    self
  }

  pub fn multi_factors(&self) -> Option<&Vec<models::V1MultiFactorType>> {
    self.multi_factors.as_ref()
  }

  pub fn reset_multi_factors(&mut self) {
    self.multi_factors = None;
  }

  pub fn set_idps(&mut self, idps: Vec<models::V1IdpLoginPolicyLink>) {
    self.idps = Some(idps);
  }

  pub fn with_idps(mut self, idps: Vec<models::V1IdpLoginPolicyLink>) -> V1LoginPolicy {
    self.idps = Some(idps);
    self
  }

  pub fn idps(&self) -> Option<&Vec<models::V1IdpLoginPolicyLink>> {
    self.idps.as_ref()
  }

  pub fn reset_idps(&mut self) {
    self.idps = None;
  }

  pub fn set_allow_domain_discovery(&mut self, allow_domain_discovery: bool) {
    self.allow_domain_discovery = Some(allow_domain_discovery);
  }

  pub fn with_allow_domain_discovery(mut self, allow_domain_discovery: bool) -> V1LoginPolicy {
    self.allow_domain_discovery = Some(allow_domain_discovery);
    self
  }

  pub fn allow_domain_discovery(&self) -> Option<&bool> {
    self.allow_domain_discovery.as_ref()
  }

  pub fn reset_allow_domain_discovery(&mut self) {
    self.allow_domain_discovery = None;
  }

  pub fn set_disable_login_with_email(&mut self, disable_login_with_email: bool) {
    self.disable_login_with_email = Some(disable_login_with_email);
  }

  pub fn with_disable_login_with_email(mut self, disable_login_with_email: bool) -> V1LoginPolicy {
    self.disable_login_with_email = Some(disable_login_with_email);
    self
  }

  pub fn disable_login_with_email(&self) -> Option<&bool> {
    self.disable_login_with_email.as_ref()
  }

  pub fn reset_disable_login_with_email(&mut self) {
    self.disable_login_with_email = None;
  }

  pub fn set_disable_login_with_phone(&mut self, disable_login_with_phone: bool) {
    self.disable_login_with_phone = Some(disable_login_with_phone);
  }

  pub fn with_disable_login_with_phone(mut self, disable_login_with_phone: bool) -> V1LoginPolicy {
    self.disable_login_with_phone = Some(disable_login_with_phone);
    self
  }

  pub fn disable_login_with_phone(&self) -> Option<&bool> {
    self.disable_login_with_phone.as_ref()
  }

  pub fn reset_disable_login_with_phone(&mut self) {
    self.disable_login_with_phone = None;
  }

  pub fn set_force_mfa_local_only(&mut self, force_mfa_local_only: bool) {
    self.force_mfa_local_only = Some(force_mfa_local_only);
  }

  pub fn with_force_mfa_local_only(mut self, force_mfa_local_only: bool) -> V1LoginPolicy {
    self.force_mfa_local_only = Some(force_mfa_local_only);
    self
  }

  pub fn force_mfa_local_only(&self) -> Option<&bool> {
    self.force_mfa_local_only.as_ref()
  }

  pub fn reset_force_mfa_local_only(&mut self) {
    self.force_mfa_local_only = None;
  }

}



