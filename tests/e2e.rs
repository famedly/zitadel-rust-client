#![allow(
	clippy::dbg_macro,
	clippy::expect_used,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	clippy::print_stdout,
	clippy::unwrap_used
)]

use std::path::Path;

use anyhow::{Ok, Result};
use test_log::{self, test};
use url::Url;
use zitadel_rust_client::v2::*;

const USER_SERVICE_PATH: &str = "tests/environment/zitadel/service-user.json";

#[tokio::test]
async fn test_e2e_create_token() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;

	assert!(Zitadel::new(url, service_account_file.to_path_buf()).await.is_ok());
	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_create_human_user() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;

	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let user = users::AddHumanUserRequest::new(
		users::SetHumanProfile::new("John".to_owned(), "Doe".to_owned()),
		users::SetHumanEmail::new("j.doe@doamin.com".to_owned()),
	);

	let resp = zitadel.create_human_user(user).await;

	if let Err(e) = &resp {
		tracing::error!("Error: {e}");
	}

	assert!(resp.is_ok());
	tracing::debug!("Response: {:?}", resp.expect("Unable to et response"));
	Ok(())
}
