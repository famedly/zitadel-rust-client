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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum V1ProjectGrantState {
	PROJECT_GRANT_STATE_UNSPECIFIED,
	PROJECT_GRANT_STATE_ACTIVE,
	PROJECT_GRANT_STATE_INACTIVE,
}
