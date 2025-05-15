// Some fields are optional in the spec, but they should not be
#![allow(missing_docs)]
use anyhow::Result;
use anyhow_trace::anyhow_trace;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::Zitadel;

#[anyhow_trace]
impl Zitadel {
	/// [Search Events](https://zitadel.com/docs/apis/resources/admin/admin-service-list-events)
	pub async fn list_events(&self, req: &V1ListEventsRequest) -> Result<Vec<V1Event>> {
		let request =
			self.client.post(self.make_url("admin/v1/events/_search")?).json(req).build()?;

		Ok(self.send_request::<V1ListEventsResponse>(request).await?.events.unwrap_or_default())
	}
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1ListEventsResponse {
	pub events: Option<Vec<V1Event>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1ListEventsRequest {
	// type: string
	// format: uint64
	// example: '2'
	/// Sequence represents the order of events. It's always counting. If asc is
	/// false, the sequence is used as lesser than filter. If asc is true
	/// sequence is used as greater than filter. If the sequence is 0 the field
	/// is ignored.
	pub sequence: Option<String>,

	// type: integer
	// format: int64
	// example: 20
	/// Maximum amount of events returned.
	pub limit: Option<i64>,

	/// default is descending sorting order
	pub asc: Option<bool>,

	// type: string
	// example: '69629023906488334'
	pub editor_user_id: Option<String>,

	// type: array
	// example:
	//   - user.human.added
	//   - user.machine
	// items:
	//   type: string
	/// The types are filtered by 'or' and must match the type exactly.
	pub event_types: Option<Vec<String>>,

	// type: string
	// example: '69629023906488334'
	pub aggregate_id: Option<String>,

	// type: array
	// example: user
	// items:
	//   type: string
	pub aggregate_types: Option<Vec<String>>,

	// type: string
	// example: '69629023906488334'
	pub resource_owner: Option<String>,

	// type: string
	// format: date-time
	// example: '2019-04-01T08:45:00.000000Z'
	/// Use `from` instead.
	pub creation_date: Option<OffsetDateTime>,

	// $ref: '#/definitions/ListEventsRequestcreation_date_range'
	pub range: Option<CreationDateRange>,

	// type: string
	// format: date-time
	// example: '2019-04-01T08:45:00.000000Z'
	/// If asc is false, the events returned are older than the UTC from date.
	/// If asc is true, the events returned are younger than from.
	#[serde(with = "time::serde::rfc3339::option")]
	pub from: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreationDateRange {
	// type: string
	// format: date-time
	// example: '2019-04-01T08:45:00.000000Z'
	/// The events returned are younger than the UTC since date
	#[serde(with = "time::serde::rfc3339::option")]
	pub since: Option<OffsetDateTime>,

	// type: string
	// format: date-time
	// example: '2019-04-01T08:45:00.000000Z'
	/// The events returned are older than the UTC until date.
	#[serde(with = "time::serde::rfc3339::option")]
	pub until: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1Event {
	// $ref: '#/definitions/v1Editor'
	pub editor: Option<V1Editor>,

	// $ref: '#/definitions/v1Aggregate'
	pub aggregate: Option<V1Aggregate>,

	// type: string
	// format: uint64
	// optional in the spec
	pub sequence: String,

	// type: string
	// format: date-time
	// example: '2019-04-01T08:45:00.000000Z'
	// optional in the spec
	/// The timestamp the event occurred
	#[serde(with = "time::serde::rfc3339")]
	pub creation_date: OffsetDateTime,

	// type: object
	// example:
	//   firstName: Gigi
	//   lastName: Giraffe
	//   userName: gigi@zitadel.com
	//   displayName: Gigi
	/// Payload contains the data of the event.
	pub payload: Option<serde_json::Value>,

	// $ref: '#/definitions/v1EventType'
	// optional in the spec
	pub r#type: V1EventType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1Editor {
	// type: string
	// example: '165617389845094785'
	// optional in the spec
	pub user_id: String,

	// type: string
	// example: Minnie Mouse
	// optional in the spec
	pub display_name: String,

	// type: string
	// example: Management-API
	// optional in the spec
	pub service: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1Aggregate {
	// type: string
	// example: '165617850743094785'
	// optional in the spec
	pub id: String,

	// $ref: '#/definitions/v1AggregateType'
	pub r#type: Option<V1AggregateType>,

	// type: string
	// example: '165617850930497249'
	// optional in the spec
	pub resource_owner: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct V1EventType {
	// type: string
	// example: user.human.added
	// optional in the spec
	pub r#type: String,

	// $ref: '#/definitions/v1LocalizedMessage'
	pub localized: Option<super::management::V1LocalizedMessage>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct V1AggregateType {
	// type: string
	// example: user
	pub r#type: Option<String>,

	// $ref: '#/definitions/v1LocalizedMessage'
	pub localized: Option<super::management::V1LocalizedMessage>,
}
