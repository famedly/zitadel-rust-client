#![allow(
	clippy::dbg_macro,
	clippy::expect_used,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	clippy::print_stdout,
	clippy::unwrap_used
)]

use std::path::Path;

use anyhow::{Context, Result};
use futures::StreamExt;
use test_log::{self, test};
use url::Url;
use users::{LastNameQuery, SearchQuery};
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
		users::SetHumanProfile::new("John".to_owned(), "create_human_user".to_owned()),
		users::SetHumanEmail::new("create_human_user@doamin.com".to_owned()),
	);

	let resp = zitadel.create_human_user(user).await;

	if let Err(e) = &resp {
		tracing::error!("Error: {e}");
	}

	assert!(resp.is_ok());
	tracing::debug!("Response: {:?}", resp.expect("Unable to et response"));
	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_list_human_users() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;

	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let mut users = Vec::new();

	for i in 0..10 {
		let user = users::AddHumanUserRequest::new(
			users::SetHumanProfile::new(format!("John_{i}"), "list_human_users".to_owned()),
			users::SetHumanEmail::new(format!("list_human_users_{i}@domain.com")),
		);
		users.push(user.clone());
		assert!(zitadel.create_human_user(user).await.is_ok());
	}

	let query =
		users::ListUsersRequest::new(vec![SearchQuery::new()
			.with_last_name_query(LastNameQuery::new("list_human_users".to_owned()))])
		.with_asc(true)
		.with_page_size(5);

	let stream = zitadel.list_users(query)?;

	let res_user_emails: Vec<_> = stream
		.map(|user| user.preferred_login_name().expect("Login of queried user not found").clone())
		.collect()
		.await;
	let user_emails: Vec<_> = users.into_iter().map(|user| user.email().email().clone()).collect();
	assert_eq!(res_user_emails, user_emails);

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_list_idp_links() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;

	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let idp_links = vec![];

	let user = users::AddHumanUserRequest::new(
		users::SetHumanProfile::new("John".to_owned(), "list_idp_links".to_owned()),
		users::SetHumanEmail::new("list_idp_links@doamin.com".to_owned()),
	)
	.with_idp_links(idp_links.clone());

	let ret = zitadel.create_human_user(user).await.context("Error creating user")?;

	let id = ret.user_id().expect("Couldn't get the user id from response");

	let query = users::UserServiceListIdpLinksBody::new().with_asc(true);

	let stream = zitadel.list_idp_links(id, query)?;

	assert_eq!(stream.count().await, idp_links.len());

	Ok(())
}
