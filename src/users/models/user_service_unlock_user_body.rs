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
#[allow(unused_imports)]
use serde_json::Value;

use crate::users::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserServiceUnlockUserBody {}

impl UserServiceUnlockUserBody {
	pub fn new() -> UserServiceUnlockUserBody {
		UserServiceUnlockUserBody {}
	}
}
