#![allow(
	clippy::dbg_macro,
	clippy::expect_used,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	clippy::print_stdout,
	clippy::unwrap_used
)]

use std::path::Path;

use anyhow::{bail, Result};
use futures::StreamExt;
use test_log::{self, test};
use url::Url;
use zitadel_rust_client::v2::{users::*, Zitadel};

const USER_SERVICE_PATH: &str = "tests/environment/zitadel/service-user.json";

/// Created an user and returns the user ID
async fn create_user(zitadel: &mut Zitadel, first_name: &str, last_name: &str) -> Result<String> {
	let user = AddHumanUserRequest::new(
		SetHumanProfile::new(first_name.to_owned(), last_name.to_owned()),
		SetHumanEmail::new(format!("{first_name}.{last_name}@domain.example")),
	);
	let ret = zitadel.create_human_user(user).await?;

	Ok(ret.user_id().expect("Couldn't get the user id from response").clone())
}

async fn tear_down(zitadel: &mut Zitadel) {
	let query = ListUsersRequest::new(vec![
		SearchQuery::new().with_type_query(TypeQuery::new(Userv2Type::Human))
	]);

	let mut stream =
		zitadel.list_users(query).expect("Error getting the list of users for tear down");

	while let Some(user) = stream.next().await {
		let id = user.user_id().expect("Couldn't get user id during tear down.");
		zitadel.delete_user(id).await.expect("Error deleting user");
	}
}

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

	let resp = create_user(&mut zitadel, "John", "Doe").await;

	if let Err(e) = &resp {
		tracing::error!("Error: {e}");
	}

	assert!(resp.is_ok());

	tear_down(&mut zitadel).await;

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
		let user = AddHumanUserRequest::new(
			SetHumanProfile::new(format!("John_{i}"), "Doe".to_owned()),
			SetHumanEmail::new(format!("j.doe_{i}@domain.example")),
		);
		users.push(user.clone());
		assert!(zitadel.create_human_user(user).await.is_ok());
	}

	let query = ListUsersRequest::new(vec![
		SearchQuery::new().with_last_name_query(LastNameQuery::new("Doe".to_owned()))
	])
	.with_asc(true)
	.with_page_size(5);

	let stream = zitadel.list_users(query)?;

	let res_user_emails: Vec<_> = stream
		.map(|user| user.preferred_login_name().expect("Login of queried user not found").clone())
		.collect()
		.await;
	let user_emails: Vec<_> = users.into_iter().map(|user| user.email().email().clone()).collect();
	assert_eq!(res_user_emails, user_emails);

	tear_down(&mut zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_list_idp_links() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let idp_links = vec![];

	let user = AddHumanUserRequest::new(
		SetHumanProfile::new("John".to_owned(), "Doe".to_owned()),
		SetHumanEmail::new("j.doe@domain.example".to_owned()),
	)
	.with_idp_links(idp_links.clone());

	let ret = zitadel.create_human_user(user).await?;
	let id = ret.user_id().expect("Couldn't get the user id from response");
	let query = UserServiceListIdpLinksBody::new().with_asc(true);

	let stream = zitadel.list_idp_links(id, query)?;

	assert_eq!(stream.count().await, idp_links.len());

	tear_down(&mut zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_change_user() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let last_name = "Doe";
	let first_name = "John";

	let id = create_user(&mut zitadel, first_name, last_name).await?;

	let ret = zitadel.get_user_by_id(&id).await?;

	let Some(rest_last_name) =
		ret.user().and_then(|u| u.human()).and_then(|u| u.profile()).and_then(|u| u.family_name())
	else {
		bail!("Couldn't get the last name of the newly created user");
	};

	assert_eq!(rest_last_name, last_name);

	let new_last_name = "NewDoe";
	let update_user = UpdateHumanUserRequest::new()
		.with_profile(SetHumanProfile::new(first_name.to_owned(), new_last_name.to_owned()));

	zitadel.update_human_user(&id, update_user).await?;

	let ret = zitadel.get_user_by_id(&id).await?;

	let Some(rest_last_name) =
		ret.user().and_then(|u| u.human()).and_then(|u| u.profile()).and_then(|u| u.family_name())
	else {
		bail!("Couldn't get the last name of the newly created user");
	};

	assert_eq!(rest_last_name, new_last_name);

	tear_down(&mut zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_activate_deactivate() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let id = create_user(&mut zitadel, "John", "Doe").await?;

	let ret = zitadel.get_user_by_id(&id).await?;
	let Some(user_state) = ret.user().and_then(|u| u.state()) else {
		bail!("Couldn't get the state of the newly created user");
	};
	assert_eq!(user_state, &UserState::Active);

	zitadel.deactivate_user(&id).await?;

	let ret = zitadel.get_user_by_id(&id).await?;
	let Some(user_state) = ret.user().and_then(|u| u.state()) else {
		bail!("Couldn't get the state of the deactivated user");
	};
	assert_eq!(user_state, &UserState::Inactive);

	zitadel.reactivate_user(&id).await?;

	let ret = zitadel.get_user_by_id(&id).await?;
	let Some(user_state) = ret.user().and_then(|u| u.state()) else {
		bail!("Couldn't get the state of the reactivated user");
	};
	assert_eq!(user_state, &UserState::Active);

	tear_down(&mut zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_delete_user() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let deleted_user_name = "John";
	let user_last_name = "Doe";

	let id = create_user(&mut zitadel, deleted_user_name, user_last_name).await?;

	create_user(&mut zitadel, "John_2", user_last_name).await?;

	let query = ListUsersRequest::new(vec![
		SearchQuery::new().with_last_name_query(LastNameQuery::new(user_last_name.to_owned()))
	])
	.with_asc(true);

	assert_eq!(zitadel.list_users(query.clone())?.count().await, 2);

	zitadel.delete_user(&id).await?;

	let user_count = zitadel
		.list_users(query.clone())?
		.inspect(|user| {
			let user_name = user
				.human()
				.and_then(|u| u.profile())
				.and_then(|u| u.given_name())
				.expect("Couldn't get user's given name");
			assert_ne!(user_name, deleted_user_name);
		})
		.count()
		.await;
	assert_eq!(user_count, 1);

	tear_down(&mut zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_user_metadata() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let metadata = vec![("key1", "value1"), ("key2", "value2")];

	let user = AddHumanUserRequest::new(
		SetHumanProfile::new("John".to_owned(), "Doe".to_owned()),
		SetHumanEmail::new("j.doe@domain.example".to_owned()),
	)
	.with_metadata(vec![SetMetadataEntry::new(metadata[0].0.to_owned(), metadata[0].1.to_owned())]);

	let id = zitadel
		.create_human_user(user)
		.await?
		.user_id()
		.cloned()
		.expect("Couldn't get the user id from response");

	zitadel.set_user_metadata(&id, metadata[1].0, metadata[1].1).await?;

	for (query_key, expected_value) in &metadata {
		let ret = zitadel.get_user_metadata(&id, query_key).await?;
		let key =
			ret.metadata().key().expect("Unable to get key from response of get_user_metadata");
		let value =
			ret.metadata().value().expect("Unable to get value from response of get_user_metadata");
		assert_eq!(query_key, key);
		assert_eq!(*expected_value, value);
	}

	zitadel.delete_user_metadata(&id, metadata[1].0).await?;
	assert!(zitadel.get_user_metadata(&id, metadata[1].0).await.is_err());

	let updated_value = "new_value";
	zitadel.set_user_metadata(&id, metadata[0].0, updated_value).await?;

	let ret = zitadel.get_user_metadata(&id, metadata[0].0).await?;
	let value = ret
		.metadata()
		.value()
		.expect("Unable to get updated value from response of get_user_metadata");
	assert_eq!(value, updated_value);

	tear_down(&mut zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_user_metadata_bulk() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let mut zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

	let metadata = vec![
		SetMetadataEntry::new("key1".to_owned(), "value1".to_owned()),
		SetMetadataEntry::new("key2".to_owned(), "value2".to_owned()),
	];

	let user = AddHumanUserRequest::new(
		SetHumanProfile::new("John".to_owned(), "Doe".to_owned()),
		SetHumanEmail::new("j.doe@domain.example".to_owned()),
	)
	.with_metadata(vec![SetMetadataEntry::new("key1".to_owned(), "old_value".to_owned())]);

	let id = zitadel
		.create_human_user(user)
		.await?
		.user_id()
		.cloned()
		.expect("Couldn't get the user id from response");

	zitadel.set_user_metadata_bulk(&id, metadata.clone()).await?;

	let stream = zitadel.list_user_metadata(
		&id,
		ListUserMetadataRequest::new(vec![
			KeyQuery::new("key".to_owned()).with_method(TextQueryMethod::Contains)
		]),
	)?;

	let res_metadata: Vec<_> = stream
		.map(|metadata| {
			(
				metadata.key().cloned().expect("Unable to get key from list_user_metadata"),
				metadata.value().expect("Unable to get value from list_user_metadata"),
			)
		})
		.collect()
		.await;
	let expected_metadata: Vec<_> = metadata
		.iter()
		.map(|metadata| {
			(
				metadata.key().clone(),
				metadata.value().expect("Unable to get value from expected metadata"),
			)
		})
		.collect();
	assert_eq!(expected_metadata, res_metadata);

	zitadel
		.delete_user_metadata_bulk(
			&id,
			metadata.iter().map(|metadata| metadata.key().clone()).collect(),
		)
		.await?;

	assert!(
		zitadel
			.list_user_metadata(
				&id,
				ListUserMetadataRequest::new(vec![
					KeyQuery::new("key".to_owned()).with_method(TextQueryMethod::Contains)
				]),
			)?
			.count()
			.await == 0
	);

	tear_down(&mut zitadel).await;

	Ok(())
}
