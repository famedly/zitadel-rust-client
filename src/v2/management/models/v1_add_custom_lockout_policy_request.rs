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
pub struct V1AddCustomLockoutPolicyRequest {
  /// When the user has reached the maximum password attempts the account will be locked, If this is set to 0 the lockout will not trigger.
  #[serde(rename = "maxPasswordAttempts")]
  max_password_attempts: Option<i64>,
  /// Maximum failed attempts for a single OTP type (TOTP, SMS, Email) before the account gets locked. Attempts are reset as soon as the OTP is entered correctly. If set to 0 the account will never be locked.
  #[serde(rename = "maxOtpAttempts")]
  max_otp_attempts: Option<i64>
}

impl V1AddCustomLockoutPolicyRequest {
  pub fn new() -> V1AddCustomLockoutPolicyRequest {
    V1AddCustomLockoutPolicyRequest {
      max_password_attempts: None,
      max_otp_attempts: None
    }
  }

  pub fn set_max_password_attempts(&mut self, max_password_attempts: i64) {
    self.max_password_attempts = Some(max_password_attempts);
  }

  pub fn with_max_password_attempts(mut self, max_password_attempts: i64) -> V1AddCustomLockoutPolicyRequest {
    self.max_password_attempts = Some(max_password_attempts);
    self
  }

  pub fn max_password_attempts(&self) -> Option<&i64> {
    self.max_password_attempts.as_ref()
  }

  pub fn reset_max_password_attempts(&mut self) {
    self.max_password_attempts = None;
  }

  pub fn set_max_otp_attempts(&mut self, max_otp_attempts: i64) {
    self.max_otp_attempts = Some(max_otp_attempts);
  }

  pub fn with_max_otp_attempts(mut self, max_otp_attempts: i64) -> V1AddCustomLockoutPolicyRequest {
    self.max_otp_attempts = Some(max_otp_attempts);
    self
  }

  pub fn max_otp_attempts(&self) -> Option<&i64> {
    self.max_otp_attempts.as_ref()
  }

  pub fn reset_max_otp_attempts(&mut self) {
    self.max_otp_attempts = None;
  }

}



