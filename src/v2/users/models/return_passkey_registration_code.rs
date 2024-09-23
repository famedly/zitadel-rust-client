/*
 * User Service
 *
 * This API is intended to manage users in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ReturnPasskeyRegistrationCode {}

impl ReturnPasskeyRegistrationCode {
	pub fn new() -> ReturnPasskeyRegistrationCode {
		ReturnPasskeyRegistrationCode {}
	}
}
