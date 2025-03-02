/*
 * Management API
 *
 * The management API is as the name states the interface where systems can
 * mutate IAM objects like organizations, projects, clients, users and so on
 * if they have the necessary access rights.
 *
 * OpenAPI spec version: 1.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
/// V1AutoLinkingOption :  - AUTO_LINKING_OPTION_UNSPECIFIED:
/// AUTO_LINKING_OPTION_UNSPECIFIED disables the auto linking prompt.  -
/// AUTO_LINKING_OPTION_USERNAME: AUTO_LINKING_OPTION_USERNAME will use the
/// username of the external user to check for a corresponding ZITADEL user.  -
/// AUTO_LINKING_OPTION_EMAIL: AUTO_LINKING_OPTION_EMAIL  will use the email of
/// the external user to check for a corresponding ZITADEL user with the same
/// verified email Note that in case multiple users match, no prompt will be
/// shown.

#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::management::models;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V1AutoLinkingOption {}

impl V1AutoLinkingOption {
	///  - AUTO_LINKING_OPTION_UNSPECIFIED: AUTO_LINKING_OPTION_UNSPECIFIED
	///    disables the auto linking prompt.  - AUTO_LINKING_OPTION_USERNAME:
	///    AUTO_LINKING_OPTION_USERNAME will use the username of the external
	///    user to check for a corresponding ZITADEL user.  -
	///    AUTO_LINKING_OPTION_EMAIL: AUTO_LINKING_OPTION_EMAIL  will use the
	///    email of the external user to check for a corresponding ZITADEL user
	///    with the same verified email Note that in case multiple users match,
	///    no prompt will be shown.
	pub fn new() -> V1AutoLinkingOption {
		V1AutoLinkingOption {}
	}
}

// TODO enum
// List of v1AutoLinkingOption
//const (
//
//
//
//)
