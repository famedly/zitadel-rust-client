//! Implementation of the client for the zitadel user's v2 api
mod models;
use anyhow::{Context, Result};
pub use models::*;

use super::Zitadel;

impl Zitadel {
	/// Crates a new human user. [Docs](https://zitadel.com/docs/apis/resources/user_service_v2/user-service-add-human-user)
	pub async fn create_human_user(
		&mut self,
		user: AddHumanUserRequest,
	) -> Result<AddHumanUserResponse> {
		let request = self
			.client
			.post(self.make_url("v2/users/human")?)
			.json(&user)
			.build()
			.context("Error building create_human_user request")?;

		self.send_request(request).await
	}
}
