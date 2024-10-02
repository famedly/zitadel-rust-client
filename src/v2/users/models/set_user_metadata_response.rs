use serde::{Deserialize, Serialize};

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SetUserMetadataResponse {
	#[serde(rename = "id")]
	id: Option<String>,
	#[serde(rename = "details")]
	details: Option<models::Details>,
}
