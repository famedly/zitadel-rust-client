#![allow(
	clippy::dbg_macro,
	clippy::expect_used,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	clippy::print_stdout,
	clippy::unwrap_used,
	missing_docs
)]

use std::{collections::HashMap, path::Path};

use anyhow_ext::{bail, Result};
use famedly_rust_utils::GenericCombinators;
use futures::{future, StreamExt, TryStreamExt};
use josekit::{jws::JwsHeader, jwt::JwtPayload};
use rand::distr::{Alphanumeric, SampleString};
use test_log::{self, test};
use time::{Duration, OffsetDateTime};
use url::Url;
use wiremock::{
	matchers::{method, path},
	Mock, MockServer, ResponseTemplate,
};
use zitadel_rust_client::v2::{
	authentication::Token, management::*, organization::*, pagination::PaginationParams, token,
	users::*, Zitadel,
};

const USER_SERVICE_PATH: &str = "tests/environment/zitadel/service-user.json";

/// Created an user and returns the user ID
async fn create_user(zitadel: &Zitadel, first_name: &str, last_name: &str) -> Result<String> {
	let user = AddHumanUserRequest::new(
		SetHumanProfile::new(first_name.to_owned(), last_name.to_owned()),
		SetHumanEmail::new(format!("{first_name}.{last_name}@domain.example")),
	);
	let ret = zitadel.create_human_user(user).await?;

	Ok(ret.user_id().expect("Couldn't get the user id from response").clone())
}

#[tracing::instrument(skip_all)]
async fn create_random_user(zitadel: &Zitadel, org_id: Option<String>) -> Result<String> {
	let req = mk_add_human_user_request().chain_opt(org_id, |r, org_id| {
		r.with_organization(Organization::new().with_org_id(org_id))
	});

	let user_id = zitadel
		.create_human_user(req.clone())
		.await
		.inspect(|u| tracing::info!("User created:\n{u:?}"))?
		.user_id()
		.unwrap()
		.clone();
	tracing::info!(user = ?req, user_id);
	Ok(user_id)
}

fn mk_add_human_user_request() -> AddHumanUserRequest {
	let first_name = Alphanumeric.sample_string(&mut rand::rng(), 12);
	let last_name = Alphanumeric.sample_string(&mut rand::rng(), 12);
	let email = format!("{}.{last_name}@domain.example", first_name.as_bytes()[0] as char);
	AddHumanUserRequest::new(SetHumanProfile::new(first_name, last_name), SetHumanEmail::new(email))
}

async fn tear_down(zitadel: &Zitadel) {
	let queries = Some(vec![SearchQuery::new().with_type_query(TypeQuery::new(Userv2Type::Human))]);

	zitadel
		.list_users(None, None, queries)
		.expect("Error getting the list of users for tear down")
		.try_for_each(|user| async move {
			let id = user.user_id().expect("Couldn't get user id during tear down.");
			zitadel.delete_user(id).await.expect("Error deleting user");
			Ok(())
		})
		.await
		.unwrap();

	zitadel
		.list_actions(None, None, Some(vec![]))
		.expect("Error getting the list of actions to tear down")
		.try_for_each(|action| async move {
			let id = action.id().expect("Couldn't get action id during tear down.");
			zitadel.delete_action(id.clone(), None).await.expect("Error deleting action");
			Ok(())
		})
		.await
		.unwrap();

	zitadel
		.list_projects(None, None, Some(vec![]))
		.expect("Error getting the list of projects to tear down")
		.try_for_each(|project| async move {
			tracing::debug!("{:?}", project);

			let id = project.id().expect("Couldn't get project id during tear down.");

			zitadel
				.list_applications(id.clone(), None, None, Some(vec![]))
				.expect("Error getting the list of applications to tear down")
				.try_for_each(|application| async move {
					zitadel
						.remove_application(
							id.clone(),
							application
								.id()
								.expect("Couldn't get application id during tear down.")
								.clone(),
							None,
						)
						.await
						.expect("Failed to remove application during teardown");
					Ok(())
				})
				.await
				.unwrap();

			zitadel.remove_project(id.clone(), None).await.expect("Error deleting project");
			Ok(())
		})
		.await
		.unwrap();
}

/// Helper macro to check the results of paginated requests.
/// ```
/// let all_users = ["u1", "u2", "u3"];
/// // Asserts that result has expected u1 and nothing else from all_users
/// assert_list!(all_users, &["u1", "u_extra"], &["u1"]);
/// ```
/// ```should_panic
/// let all_users = ["u1", "u2", "u3"];
/// // Panics because the result doesn't have u2
/// assert_list!(all_users, &["u1", "u_extra"], &["u1", "u2"]);
/// ```
/// ```should_panic
/// let all_users = ["u1", "u2", "u3"];
/// // Panics because the result has unexpected u2
/// assert_list!(all_users, &["u1", "u2"], &["u1"]);
/// ```
macro_rules! assert_list {
	($all_items:expr, $items:expr, $items_must_be:expr) => {
		let items = $items;
		let items_must_be = $items_must_be;
		items_must_be.iter().for_each(|item| {
			assert!(
				items.iter().find(|u| item == u).is_some(),
				"{}\n=\n{items:?}\nmust contain {:?}",
				stringify!($items),
				item
			)
		});
		let items_must_not_be = $all_items
			.iter()
			.filter(|item| items.iter().find(|u| &u == item).is_none())
			.collect::<Vec<_>>();
		items_must_not_be.iter().for_each(|item| {
			assert!(
				items.iter().find(|u| item == &u).is_none(),
				"{}\n=\n{items:?}\nmust not contain {:?}",
				stringify!($items),
				item
			)
		});
	};
}

#[tokio::test]
async fn test_e2e_create_token() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;

	assert!(Zitadel::new(url, service_account_file.to_path_buf(), None).await.is_ok());
	Ok(())
}

async fn mk_zitadel_client() -> Result<Zitadel> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	Zitadel::new(url, service_account_file.to_path_buf(), None).await
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
#[allow(trivial_casts)]
async fn test_e2e_role_and_membership_methods() -> Result<()> {
	let zitadel = mk_zitadel_client().await?;
	let admin = create_random_user(&zitadel, None).await?;

	let admins = vec![AddOrganizationRequestAdmin::new().with_user_id(admin)];

	let roles = |role: &str| vec![role.to_owned()];

	// Create org1 with 2 projects in it
	let query = V2AddOrganizationRequest::new("A".to_owned()).with_admins(admins.clone());
	let org1 =
		zitadel.create_organization_with_admin(query).await?.organization_id().unwrap().clone();

	let query = V1AddProjectRequest::new("A1".to_owned());
	let project11 = zitadel.create_project(query, Some(org1.clone())).await?.id().unwrap().clone();
	zitadel
		.add_project_role(Some(org1.clone()), &project11, "User".into(), "A user".into(), None)
		.await?;

	let query = V1AddProjectRequest::new("A2".to_owned());
	let project12 = zitadel.create_project(query, Some(org1.clone())).await?.id().unwrap().clone();
	zitadel
		.add_project_role(Some(org1.clone()), &project12, "User".into(), "A user".into(), None)
		.await?;

	// A user member of the org
	let user11 = create_random_user(&zitadel, None).await?;
	zitadel.add_organization_member(Some(org1.clone()), user11.clone(), roles("ORG_OWNER")).await?;

	// A user member of the org and project11
	let user12 = create_random_user(&zitadel, None).await?;
	zitadel.add_organization_member(Some(org1.clone()), user12.clone(), roles("ORG_OWNER")).await?;
	zitadel
		.add_project_member(Some(org1.clone()), &project11, user12.clone(), roles("PROJECT_OWNER"))
		.await?;

	// A user member of the org and is granted project12
	let user13 = create_random_user(&zitadel, None).await?;
	zitadel.add_organization_member(Some(org1.clone()), user13.clone(), roles("ORG_OWNER")).await?;

	// A user who is member but not "member" of the org
	let user14 = create_random_user(&zitadel, Some(org1.clone())).await?;

	let _user13_project12_grant = zitadel
		.add_user_grant(
			Some(org1.clone()),
			&user13,
			project12.clone(),
			None,
			Some(vec!["User".into()]),
		)
		.await?
		.user_grant_id()
		.unwrap()
		.clone();

	// Create org2, add project12 (from org1) grant
	let query = V2AddOrganizationRequest::new("B".to_owned()).with_admins(admins);
	let org2 =
		zitadel.create_organization_with_admin(query).await?.organization_id().unwrap().clone();

	let project_grant_id = zitadel
		.add_project_grant(None, &project12, org2.clone(), None)
		.await?
		.grant_id()
		.unwrap()
		.clone();

	// User member of the org
	let user21 = create_random_user(&zitadel, None).await?;
	zitadel.add_organization_member(Some(org2.clone()), user21.clone(), roles("ORG_OWNER")).await?;

	// User member of the org and is granted project12 via user grant
	let user22 = create_random_user(&zitadel, None).await?;
	zitadel.add_organization_member(Some(org2.clone()), user22.clone(), roles("ORG_OWNER")).await?;

	let _user22_project12_grant = zitadel
		.add_user_grant(
			Some(org2.clone()),
			&user22.clone(),
			project12.clone(),
			Some(project_grant_id.clone()),
			None,
		)
		.await?;

	// Use member of the org, member of the project grant
	let user23 = create_random_user(&zitadel, None).await?;
	zitadel.add_organization_member(Some(org2.clone()), user23.clone(), roles("ORG_OWNER")).await?;

	zitadel
		.add_project_grant_member(
			Some(org2.clone()),
			&project12,
			&project_grant_id,
			user23.clone(),
			roles("PROJECT_GRANT_OWNER"),
		)
		.await?;

	// A user who is member but not "member" of the org and is granted project12 via
	// user grant
	let user24 = create_random_user(&zitadel, Some(org2.clone())).await?;

	let _user24_project12_grant = zitadel
		.add_user_grant(
			Some(org2.clone()),
			&user24.clone(),
			project12.clone(),
			Some(project_grant_id.clone()),
			None,
		)
		.await?;

	// Asserts

	tracing::info!(user11, user12, user13, user14, user21, user22, user23, user24);

	let all_users = [&user11, &user12, &user13, &user14, &user21, &user22, &user23, &user24];

	assert_list!(
		all_users,
		zitadel
			.list_organization_members(Some(org1.clone()), None, Some(Vec::new()))?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[&user11, &user12, &user13]
	);

	assert_list!(
		all_users,
		zitadel
			.list_organization_members(Some(org2.clone()), None, Some(Vec::new()))?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[&user21, &user22, &user23]
	);

	assert_list!(
		all_users,
		zitadel
			.list_project_members(Some(org1.clone()), &project11, None, Some(Vec::new()))?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[&user12]
	);

	assert_list!(
		all_users,
		zitadel
			.list_project_grant_members(
				Some(org2.clone()),
				&project12,
				&project_grant_id,
				None,
				Some(Vec::new())
			)?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[&user23]
	);

	assert_list!(
		all_users,
		zitadel
			.list_project_grant_members(
				Some(org1.clone()),
				&project12,
				&project_grant_id,
				None,
				Some(Vec::new())
			)?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[] as &[&str]
	);

	assert_list!(
		all_users,
		zitadel
			.search_user_grants(
				Some(org1.clone()),
				None,
				Some(vec![
					V1UserGrantQuery::ProjectId {
						project_id_query: V1UserGrantProjectIdQuery::new()
							.with_project_id(project12.clone())
					},
					V1UserGrantQuery::UserType {
						user_type_query: V1UserGrantUserTypeQuery::new()
							.with__type(Userv1Type::Human)
					}
				])
			)?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[&user13]
	);

	assert_list!(
		all_users,
		zitadel
			.search_user_grants(
				Some(org2.clone()),
				None,
				Some(vec![V1UserGrantQuery::ProjectId {
					project_id_query: (V1UserGrantProjectIdQuery::new()
						.with_project_id(project12.clone()))
				}])
			)?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[&user22, &user24]
	);

	assert_list!(
		all_users,
		zitadel
			.search_user_grants(
				Some(org2.clone()),
				None,
				Some(vec![V1UserGrantQuery::ProjectGrantId {
					project_grant_id_query: (V1UserGrantProjectGrantIdQuery::new()
						.with_project_grant_id(project_grant_id.clone()))
				}])
			)?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[&user22, &user24]
	);

	assert_list!(
		all_users,
		zitadel
			.search_user_grants(
				None,
				None,
				Some(vec![V1UserGrantQuery::ProjectId {
					project_id_query: (V1UserGrantProjectIdQuery::new()
						.with_project_id(project12.clone()))
				}])
			)?
			.and_then(|x| future::ok(x.user_id().unwrap().clone()))
			.try_collect::<Vec<_>>()
			.await
			.unwrap(),
		&[] as &[&str]
	);

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_create_organization() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let admins = vec![
		(AddOrganizationRequestAdmin::new().with_human(AddHumanUserRequest::new(
			SetHumanProfile::new("Max".to_owned(), "Adminmann".to_owned()),
			SetHumanEmail::new("m.adminmann@domain.example".to_owned()),
		))),
	];

	let query = V2AddOrganizationRequest::new("test_org".to_owned()).with_admins(admins);
	let res = zitadel.create_organization_with_admin(query).await;

	// TODO: Once we have a list org method, we should check that the
	// org was actually created
	assert!(res.is_ok());

	// Ensure the admin user was actually created
	assert_eq!(
		zitadel
			.list_users(
				None,
				None,
				Some(vec![SearchQuery::new()
					.with_last_name_query(LastNameQuery::new("Adminmann".to_owned()))])
			)?
			.count()
			.await,
		1
	);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_create_application() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let project =
		zitadel.create_project(V1AddProjectRequest::new("TestProject".to_owned()), None).await?;

	let project_id = project.id().expect("Must have created a project with a valid ID");

	zitadel
		.create_application(
			project_id.to_owned(),
			ManagementServiceAddApiAppBody::new("TestApp".to_owned()),
			None,
		)
		.await?;

	let stream = zitadel.list_applications(project_id.to_owned(), None, None, Some(vec![]))?;

	let apps: Vec<String> = stream
		.and_then(|app| future::ok(app.name().expect("App name must be set").clone()))
		.try_collect::<Vec<_>>()
		.await
		.unwrap();
	assert_eq!(apps, ["TestApp"]);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_list_applications() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	// TODO: Figure out why the teardown fails to delete this project
	let project =
		zitadel.create_project(V1AddProjectRequest::new("TestProject2".to_owned()), None).await?;

	let project_id = project.id().expect("Must have created a project with a valid ID");

	let mut application_names = Vec::new();

	for i in 0..10 {
		let name = format!("TestApp{i}");
		let application = ManagementServiceAddApiAppBody::new(name.clone());
		application_names.push(name);
		assert!(zitadel.create_application(project_id.to_owned(), application, None).await.is_ok());
	}

	let stream = zitadel.list_applications(project_id.to_owned(), None, None, Some(vec![]))?;

	let found_application_names: Vec<String> = stream
		.and_then(|app| future::ok(app.name().expect("No name found for action").clone()))
		.try_collect::<Vec<_>>()
		.await
		.unwrap();
	assert_eq!(found_application_names, application_names);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_create_action() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let res = zitadel
		.create_action(
			V1CreateActionRequest::new(
				"test-action".to_owned(),
				"console.log('test-action')".to_owned(),
			),
			None,
		)
		.await;

	assert!(res.is_ok());

	let stream = zitadel.list_actions(
		None,
		None,
		Some(vec![V1ActionQuery::new()
			.with_action_name_query(V1ActionNameQuery::new().with_name("test-action".to_owned()))]),
	)?;

	let actions = stream
		.and_then(|action| future::ok(action.name().expect("No name found for action").clone()))
		.try_collect::<Vec<String>>()
		.await
		.unwrap();
	assert_eq!(actions, vec!["test-action"]);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_list_actions() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let mut action_names = Vec::new();

	for i in 0..10 {
		let name = format!("test-action-{i}");

		let action =
			V1CreateActionRequest::new(name.clone(), "console.log('test-action')".to_owned());

		action_names.push(name);
		assert!(zitadel.create_action(action, None).await.is_ok());
	}

	let stream = zitadel.list_actions(None, None, None)?;

	let found_action_names = stream
		.and_then(|action| future::ok(action.name().expect("No name found for action").clone()))
		.try_collect::<Vec<String>>()
		.await
		.unwrap();
	assert_eq!(found_action_names, action_names);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_update_action() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let res = zitadel
		.create_action(
			V1CreateActionRequest::new(
				"test-action".to_owned(),
				"console.log('test-action')".to_owned(),
			),
			None,
		)
		.await?;

	let action_id = res.id().expect("Must have created an action with a valid ID");

	zitadel
		.update_action(
			action_id.to_owned(),
			ManagementServiceUpdateActionBody::new(
				"test-action".to_owned(),
				"console.log('test-action-update')".to_owned(),
			),
			None,
		)
		.await?;

	let mut stream = zitadel.list_actions(
		None,
		None,
		Some(vec![V1ActionQuery::new()
			.with_action_name_query(V1ActionNameQuery::new().with_name("test-action".to_owned()))]),
	)?;

	let action = stream.next().await.expect("Action must exist").unwrap();

	assert_eq!(action.name(), Some(&"test-action".to_owned()));
	assert_eq!(action.script(), Some(&"console.log('test-action-update')".to_owned()));

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_delete_action() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let res = zitadel
		.create_action(
			V1CreateActionRequest::new("test-action".to_owned(), "console.log(test)".to_owned()),
			None,
		)
		.await?;

	let action_id = res.id().expect("Must have created an action with a valid ID");

	zitadel.delete_action(action_id.to_owned(), None).await?;

	let stream = zitadel.list_actions(
		None,
		None,
		Some(vec![V1ActionQuery::new()
			.with_action_name_query(V1ActionNameQuery::new().with_name("test-action".to_owned()))]),
	)?;

	assert_eq!(stream.count().await, 0);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "trace")]
async fn test_e2e_set_trigger_actions() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let res = zitadel
		.create_action(
			V1CreateActionRequest::new("test-action".to_owned(), "console.log(test)".to_owned()),
			None,
		)
		.await?;

	let action_id = res.id().expect("Must have created an action with a valid ID");

	zitadel
		.set_trigger_actions(
			1,
			1,
			ManagementServiceSetTriggerActionsBody::new()
				.with_action_ids(vec![action_id.to_owned()]),
			None,
		)
		.await?;

	let res = zitadel.get_flow(1, None).await?;
	assert_eq!(
		res.flow().and_then(|flow| flow._type().and_then(|t| t.id())),
		Some(&"1".to_owned())
	);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_create_human_user() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let resp = create_user(&zitadel, "John", "Doe").await;

	if let Err(e) = &resp {
		tracing::error!("Error: {e}");
	}

	assert!(resp.is_ok());

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_list_human_users() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let mut users = Vec::new();

	for i in 0..10 {
		let user = AddHumanUserRequest::new(
			SetHumanProfile::new(format!("John_{i}"), "Doe".to_owned()),
			SetHumanEmail::new(format!("j.doe_{i}@domain.example")),
		);
		users.push(user.clone());
		assert!(zitadel.create_human_user(user).await.is_ok());
	}

	let queries =
		Some(vec![SearchQuery::new().with_last_name_query(LastNameQuery::new("Doe".to_owned()))]);
	let pagination_params = Some(PaginationParams::DEFAULT.with_asc(true).with_page_size(5));

	let stream = zitadel.list_users(pagination_params, None, queries)?;

	let res_user_emails: Vec<_> = stream
		.and_then(|user| {
			future::ok(
				user.preferred_login_name().expect("Login of queried user not found").clone(),
			)
		})
		.try_collect::<Vec<_>>()
		.await
		.unwrap();
	let user_emails: Vec<_> = users.into_iter().map(|user| user.email().email().clone()).collect();
	assert_eq!(res_user_emails, user_emails);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_list_idp_links() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let idp_links = vec![];

	let user = AddHumanUserRequest::new(
		SetHumanProfile::new("John".to_owned(), "Doe".to_owned()),
		SetHumanEmail::new("j.doe@domain.example".to_owned()),
	)
	.with_idp_links(idp_links.clone());

	let ret = zitadel.create_human_user(user).await?;
	let id = ret.user_id().expect("Couldn't get the user id from response");
	let pagination_params = Some(PaginationParams::DEFAULT.with_asc(true));

	let stream = zitadel.list_idp_links(id, pagination_params)?;

	assert_eq!(stream.count().await, idp_links.len());

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_change_user() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let last_name = "Doe";
	let first_name = "John";

	let id = create_user(&zitadel, first_name, last_name).await?;

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

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_activate_deactivate() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let id = create_user(&zitadel, "John", "Doe").await?;

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

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_delete_user() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	let deleted_user_name = "John";
	let user_last_name = "Doe";

	let id = create_user(&zitadel, deleted_user_name, user_last_name).await?;

	create_user(&zitadel, "John_2", user_last_name).await?;

	let queries = Some(vec![
		SearchQuery::new().with_last_name_query(LastNameQuery::new(user_last_name.to_owned()))
	]);
	let pagination_params = Some(PaginationParams::DEFAULT.with_asc(true));

	assert_eq!(
		zitadel.list_users(pagination_params.clone(), None, queries.clone())?.count().await,
		2
	);

	zitadel.delete_user(&id).await?;

	let user_count = zitadel
		.list_users(pagination_params, None, queries)?
		.inspect_ok(|user| {
			let user_name = user
				.human()
				.and_then(|u| u.profile())
				.and_then(|u| u.given_name())
				.expect("Couldn't get user's given name");
			assert_ne!(user_name, deleted_user_name);
		})
		.try_collect::<Vec<_>>()
		.await
		.unwrap();
	assert_eq!(user_count.len(), 1);

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_user_metadata() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

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

	tear_down(&zitadel).await;

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_user_metadata_bulk() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;
	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

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
		None,
		Some(vec![KeyQuery::new("key".to_owned()).with_method(TextQueryMethod::Contains)]),
	)?;

	let res_metadata: Vec<_> = stream
		.map_ok(|metadata| {
			(
				metadata.key().cloned().expect("Unable to get key from list_user_metadata"),
				metadata.value().expect("Unable to get value from list_user_metadata"),
			)
		})
		.try_collect::<Vec<_>>()
		.await
		.unwrap();
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
				None,
				Some(vec![KeyQuery::new("key".to_owned()).with_method(TextQueryMethod::Contains)]),
			)?
			.count()
			.await == 0
	);

	tear_down(&zitadel).await;

	Ok(())
}

#[tokio::test]
async fn test_e2e_simple_token_verification() -> Result<()> {
	let url = Url::parse("http://localhost:8080")?;
	let service_account_file = Path::new(USER_SERVICE_PATH).to_path_buf();
	let client = reqwest::Client::new();

	// Work around to get set the token type to JWT for the service account
	// TODO: Remove it once we have the zitadel bootstrap in place
	let mut put_url = url.clone();
	let service_account: HashMap<String, serde_json::Value> =
		serde_json::from_str(tokio::fs::read_to_string(&service_account_file).await?.as_ref())?;
	put_url.set_path(&format!(
		"management/v1/users/{}/machine",
		service_account.get("userId").and_then(|v| v.as_str()).unwrap()
	));
	let token = Token::new(url.clone(), &service_account_file, client.clone(), None, None)
		.await?
		.token()
		.await?;
	let r = client
		.put(put_url)
		.bearer_auth(token)
		.body(r#"{"accessTokenType": "ACCESS_TOKEN_TYPE_JWT", "name":"Admin"}"#)
		.send()
		.await?;
	println!("body:\n{}", r.text().await?);

	let token =
		Token::new(url.clone(), &service_account_file, client, None, None).await?.token().await?;
	let token_verifier = token::ZitadelJWTVerifier::new(url);

	assert!(token_verifier.verify(token).await.is_ok());

	Ok(())
}

#[tokio::test]
async fn test_e2e_token_verification_negative() -> Result<()> {
	let mock = MockServer::start().await;
	let mut jwks = josekit::jwk::JwkSet::from_bytes(r#"{"keys":[]}"#)?;
	let private_key = josekit::jwk::Jwk::generate_rsa_key(2048)?;
	let key_id = "123456";
	let mut public_key = private_key.to_public_key()?;
	public_key.set_key_id(key_id);
	jwks.push_key(public_key);

	let signer = josekit::jws::RS256.signer_from_jwk(&private_key)?;

	let mut header = JwsHeader::new();
	header.set_key_id(key_id);
	header.set_token_type("JWT");
	header.set_algorithm("RS256");

	Mock::given(method("GET"))
		.and(path("/oauth/v2/keys"))
		.respond_with(
			ResponseTemplate::new(200)
				.set_body_string(jwks.to_string())
				.insert_header("Cache-control", "max-age=60"),
		)
		.mount(&mock)
		.await;

	let url = Url::parse(&mock.uri())?;
	let token_verifier = token::ZitadelJWTVerifier::new(url);

	let now = OffsetDateTime::now_utc();

	let mut base_payload = JwtPayload::new();
	base_payload.set_issuer(mock.uri());
	base_payload.set_expires_at(&(now + Duration::minutes(10)).into());
	base_payload.set_not_before(&(now - Duration::minutes(10)).into());
	base_payload.set_claim("test_claim", Some("test_value".into()))?;

	let mut jwts: Vec<(&JwtPayload, &JwsHeader)> = Vec::new();

	let mut header_wrong_kid = header.clone();
	header_wrong_kid.set_key_id("987654");
	jwts.push((&base_payload, &header_wrong_kid));

	let mut payload_expired = base_payload.clone();
	payload_expired.set_expires_at(&(now - Duration::minutes(5)).into());
	jwts.push((&payload_expired, &header));

	let mut payload_future = base_payload.clone();
	payload_future.set_issued_at(&(now + Duration::minutes(1)).into());
	jwts.push((&payload_future, &header));

	let mut payload_wrong_issuer = base_payload.clone();
	payload_wrong_issuer.set_issuer("Wrong_issuer");
	jwts.push((&payload_wrong_issuer, &header));

	for (payload, header) in jwts {
		let jwt = josekit::jwt::encode_with_signer(payload, header, &signer)?;
		assert!(token_verifier.verify(jwt).await.is_err());
	}

	let wrong_private_key = josekit::jwk::Jwk::generate_rsa_key(2048)?;
	let wrong_signer = josekit::jws::RS256.signer_from_jwk(&wrong_private_key)?;
	let jwt = josekit::jwt::encode_with_signer(&base_payload, &header, &wrong_signer)?;
	assert!(token_verifier.verify(jwt).await.is_err());

	Ok(())
}

#[tokio::test]
async fn test_e2e_token_verification_positive() -> Result<()> {
	let mock = MockServer::start().await;
	let mut jwks = josekit::jwk::JwkSet::from_bytes(r#"{"keys":[]}"#)?;
	let private_key = josekit::jwk::Jwk::generate_rsa_key(2048)?;
	let key_id = "123456";
	let mut public_key = private_key.to_public_key()?;
	public_key.set_key_id(key_id);
	jwks.push_key(public_key);

	Mock::given(method("GET"))
		.and(path("/oauth/v2/keys"))
		.respond_with(
			ResponseTemplate::new(200)
				.set_body_string(jwks.to_string())
				.insert_header("Cache-control", "max-age=60"),
		)
		.mount(&mock)
		.await;

	let now = OffsetDateTime::now_utc();

	let mut base_payload = JwtPayload::new();
	base_payload.set_issuer(mock.uri());
	base_payload.set_expires_at(&(now + Duration::minutes(10)).into());
	base_payload.set_issued_at(&(now - Duration::minutes(10)).into());
	base_payload.set_claim("test_claim", Some("test_value".into()))?;

	let mut header = JwsHeader::new();
	header.set_key_id(key_id);
	header.set_token_type("JWT");
	header.set_algorithm("RS256");

	let signer = josekit::jws::RS256.signer_from_jwk(&private_key)?;
	let jwt = josekit::jwt::encode_with_signer(&base_payload, &header, &signer)?;

	let url = Url::parse(&mock.uri())?;
	let token_verifier = token::ZitadelJWTVerifier::new(url);
	let verification_result = token_verifier.verify(jwt).await;
	println!("verification_result: {verification_result:?}");
	assert!(verification_result.is_ok());

	Ok(())
}

#[test(tokio::test)]
#[test_log(default_log_filter = "debug")]
async fn test_e2e_organization_scoped_operations() -> Result<()> {
	let service_account_file = Path::new(USER_SERVICE_PATH);
	let url = Url::parse("http://localhost:8080")?;

	let zitadel = Zitadel::new(url, service_account_file.to_path_buf(), None).await?;

	// Create two organizations to test scoping
	let org_name = "test-org-scoped";
	let org_response = zitadel
		.create_organization_with_admin(V2AddOrganizationRequest::new(org_name.to_owned()))
		.await
		.expect("Failed to create organization");
	let org_id = org_response
		.organization_id()
		.expect("Organization ID should be present in response")
		.to_owned();

	// Create a project in this organization
	let project = zitadel
		.create_project(
			V1AddProjectRequest::new("OrgScopedProject".to_owned()),
			Some(org_id.clone()),
		)
		.await
		.expect("Failed to create project");
	let project_id = project.id().expect("Project ID should be present in response").to_owned();

	// Verify project exists in the correct organization
	let project_request = vec![V1ProjectQuery::new()
		.with_name_query(V1ProjectNameQuery::new().with_name("OrgScopedProject".to_owned()))];

	// Should find no projects in default org
	let projects_default_org = zitadel
		.list_projects(None, None, Some(project_request.clone()))
		.expect("Failed to list projects in default organization")
		.collect::<Vec<_>>()
		.await;
	assert!(projects_default_org.is_empty());

	// Should find one project in specified org
	let projects_specified_org = zitadel
		.list_projects(Some(org_id.clone()), None, Some(project_request))
		.expect("Failed to list projects in specified organization")
		.collect::<Vec<_>>()
		.await;
	assert_eq!(projects_specified_org.len(), 1);

	// Create an application in this organization's project
	let app = zitadel
		.create_application(
			project_id.clone(),
			ManagementServiceAddApiAppBody::new("OrgScopedApp".to_owned()),
			Some(org_id.clone()),
		)
		.await
		.expect("Failed to create application");
	let app_id = app.app_id().expect("Application ID should be present in response").to_owned();

	// Verify application exists in the correct project/organization
	// NOTE: Maybe a Zitadel bug -> request uses the project ID to search
	// for applications and disregards the org_id
	let app_request = vec![V1AppQuery::new()
		.with_name_query(V1AppNameQuery::new().with_name("OrgScopedApp".to_owned()))];
	let apps = zitadel
		.list_applications(project_id.clone(), Some(org_id.clone()), None, Some(app_request))
		.expect("Failed to list applications")
		.collect::<Vec<_>>()
		.await;
	assert_eq!(apps.len(), 1);

	// Create an action in this organization
	let action = zitadel
		.create_action(
			V1CreateActionRequest::new(
				"org-scoped-action".to_owned(),
				"console.log('org-scoped')".to_owned(),
			),
			Some(org_id.clone()),
		)
		.await
		.expect("Failed to create action");
	let action_id = action.id().expect("Action ID should be present in response").to_owned();

	// Verify we can find the action when searching with org_id
	let actions_request = vec![V1ActionQuery::new().with_action_name_query(
		V1ActionNameQuery::new().with_name("org-scoped-action".to_owned()),
	)];

	// Should find no actions in default org
	let actions_default_org = zitadel
		.list_actions(None, None, Some(actions_request.clone()))
		.expect("Failed to list actions in default organization")
		.collect::<Vec<_>>()
		.await;
	assert!(actions_default_org.is_empty());

	// Should find one action in specified org
	let actions_specified_org = zitadel
		.list_actions(Some(org_id.clone()), None, Some(actions_request))
		.expect("Failed to list actions in specified organization")
		.collect::<Vec<_>>()
		.await;
	assert_eq!(actions_specified_org.len(), 1);

	// Try to update the action with wrong organization ID - should fail
	let update_result = zitadel
		.update_action(
			action_id.clone(),
			ManagementServiceUpdateActionBody::new(
				"org-scoped-action".to_owned(),
				"console.log('should-not-update')".to_owned(),
			),
			None,
		)
		.await;
	assert!(update_result.is_err(), "Update with wrong org ID should fail");

	// Update the action with correct organization ID
	zitadel
		.update_action(
			action_id.clone(),
			ManagementServiceUpdateActionBody::new(
				"org-scoped-action".to_owned(),
				"console.log('updated-org-scoped')".to_owned(),
			),
			Some(org_id.clone()),
		)
		.await
		.expect("Failed to update action");

	// Verify the action was updated with the correct content
	let actions_request = vec![V1ActionQuery::new().with_action_name_query(
		V1ActionNameQuery::new().with_name("org-scoped-action".to_owned()),
	)];
	let updated_actions = zitadel
		.list_actions(Some(org_id.clone()), None, Some(actions_request))
		.expect("Failed to list actions")
		.try_collect::<Vec<_>>()
		.await
		.unwrap();
	assert_eq!(updated_actions.len(), 1);
	assert_eq!(updated_actions[0].script(), Some(&"console.log('updated-org-scoped')".to_owned()));

	// Set up flow triggers with the action
	zitadel
		.set_trigger_actions(
			1,
			1,
			ManagementServiceSetTriggerActionsBody::new().with_action_ids(vec![action_id.clone()]),
			Some(org_id.clone()),
		)
		.await
		.expect("Failed to set up flow triggers");

	// Try to set up flow triggers with wrong organization ID - should fail
	let trigger_result = zitadel
		.set_trigger_actions(
			1,
			1,
			ManagementServiceSetTriggerActionsBody::new().with_action_ids(vec![action_id.clone()]),
			None,
		)
		.await;
	assert!(trigger_result.is_err(), "Setting triggers with wrong org ID should fail");

	// Verify flow is set up correctly
	let flow = zitadel.get_flow(1, Some(org_id.clone())).await.expect("Failed to get flow");
	assert_eq!(flow.flow().and_then(|f| f._type().and_then(|t| t.id())), Some(&"1".to_owned()));

	// Manual clean up,
	// because the tear_down function cleans up default org namespace only
	zitadel.delete_action(action_id, Some(org_id.clone())).await.expect("Failed to delete action");
	zitadel
		.remove_application(project_id.clone(), app_id, Some(org_id.clone()))
		.await
		.expect("Failed to delete application");
	zitadel
		.remove_project(project_id, Some(org_id.clone()))
		.await
		.expect("Failed to delete project");

	tear_down(&zitadel).await;

	Ok(())
}
