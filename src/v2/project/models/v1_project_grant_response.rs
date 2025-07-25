use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::project::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ProjectGrantResponse {
	#[serde(rename = "grantedProject")]
	pub granted_project: models::V1ProjectGrant,
}
