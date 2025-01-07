/*
 * Organization Service
 *
 * This API is intended to manage organizations in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::organization::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2OrganizationState {}

impl V2OrganizationState {
	pub fn new() -> V2OrganizationState {
		V2OrganizationState {}
	}
}

// TODO enum
// List of v2OrganizationState
//const (
//
//
//
//)
