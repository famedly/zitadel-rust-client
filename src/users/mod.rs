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

#[cfg(test)]
mod test {
	#![allow(clippy::expect_used)]
	use std::path::Path;

	use anyhow::Result;
	use test_log::{self, test};
	use url::Url;

	use super::*;

	#[test(tokio::test)]
	#[test_log(default_log_filter = "debug")]
	async fn create_human_user() -> Result<()> {
		let service_account_file = Path::new("tests/zitadel/service-user.json");
		let url = Url::parse("http://localhost:8080")?;

		let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

		let user = AddHumanUserRequest::new(
			SetHumanProfile::new("John".to_owned(), "Doe".to_owned()),
			SetHumanEmail::new("j.doe@doamin.com".to_owned()),
		);

		let resp = zitadel.create_human_user(user).await;

		if let Err(e) = &resp {
			tracing::error!("Error: {e}");
		}

		assert!(resp.is_ok());
		tracing::debug!("Response: {:?}", resp.expect("Unable to et response"));
		Ok(())
	}
}
