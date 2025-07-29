// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Zitadel API types

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use time::OffsetDateTime;

use super::error::Error::{self, EventConversion};

/// Custom Zitadel event type, that captures the information that we need, but
/// not all available information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	/// The sequence number of the event.
	pub sequence: u64,
	/// Creation date of the event.
	pub creation_date: OffsetDateTime,
	/// Event type.
	pub r#type: String,
	/// The object that was modified.
	pub aggregate: Option<Aggregate>,
	/// The editor that caused the event.
	pub editor: Option<Editor>,
	/// Payload of the event, hold arbitrary data correspondent to the event
	/// type.
	pub payload: BTreeMap<String, Value>,
}

/// Zitadel event editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editor {
	/// The user ID of the editor.
	pub user_id: String,
	/// The display name of the editor.
	pub display_name: String,
	/// The service used by the editor.
	/// For example, Management-API or Admin-API.
	pub service: String,
}

/// Zitadel event aggregate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregate {
	/// Type of the object.
	pub r#type: Option<String>,
	/// ID of the object.
	pub id: String,
	/// Resource owner / Organization ID of this object.
	pub resource_owner: String,
}

impl TryFrom<zitadel::api::zitadel::event::v1::Event> for Event {
	type Error = Error;

	fn try_from(event: zitadel::api::zitadel::event::v1::Event) -> Result<Self, Self::Error> {
		let timestamp = event.creation_date.ok_or(EventConversion("Bad date".into()))?;
		let creation_date = OffsetDateTime::from_unix_timestamp_nanos(
			i128::from(timestamp.seconds) * 1_000_000_000 + i128::from(timestamp.nanos),
		)
		.map_err(|e| EventConversion(e.to_string()))?;

		Ok(Self {
			sequence: event.sequence,
			creation_date,
			r#type: event.r#type.ok_or(EventConversion("No type in event".into()))?.r#type,
			aggregate: event.aggregate.map(Into::into),
			editor: event.editor.map(Into::into),
			payload: event.payload.map_or(Ok(BTreeMap::default()), convert_struct)?,
		})
	}
}

impl From<zitadel::api::zitadel::event::v1::Editor> for Editor {
	fn from(editor: zitadel::api::zitadel::event::v1::Editor) -> Self {
		Self { user_id: editor.user_id, display_name: editor.display_name, service: editor.service }
	}
}

impl From<zitadel::api::zitadel::event::v1::Aggregate> for Aggregate {
	fn from(aggregate: zitadel::api::zitadel::event::v1::Aggregate) -> Self {
		Self {
			id: aggregate.id,
			r#type: aggregate.r#type.map(|ty| ty.r#type),
			resource_owner: aggregate.resource_owner,
		}
	}
}

/// Convert a `pbjson` `Struct` to a map of `serde_json` `Value`s.
fn convert_struct(pbjson_struct: pbjson_types::Struct) -> Result<BTreeMap<String, Value>, Error> {
	pbjson_struct
		.fields
		.into_iter()
		.map(|(key, pbjson_value)| Ok((key, convert_value(pbjson_value)?)))
		.collect()
}

/// Convert a `pbjson::Value` to a `serde_json::Value`.
fn convert_value(pbjson_value: pbjson_types::Value) -> Result<Value, Error> {
	Ok(match pbjson_value.kind {
		None => Value::Null,
		Some(pbjson_types::value::Kind::NullValue(_)) => Value::Null,
		Some(pbjson_types::value::Kind::NumberValue(num)) => {
			Value::Number(Number::from_f64(num).ok_or(EventConversion("Bad f64 value".into()))?)
		}
		Some(pbjson_types::value::Kind::StringValue(s)) => Value::String(
			// The Zitadel API seems to at least sometimes return string values wrapped in
			// extraneous quotes
			s.strip_prefix('"').unwrap_or(&s).strip_suffix('"').unwrap_or(&s).to_owned(),
		),
		Some(pbjson_types::value::Kind::BoolValue(b)) => Value::Bool(b),
		Some(pbjson_types::value::Kind::StructValue(st)) => {
			Value::Object(convert_struct(st)?.into_iter().collect())
		}
		Some(pbjson_types::value::Kind::ListValue(a)) => {
			Value::Array(a.values.into_iter().map(convert_value).collect::<Result<_, _>>()?)
		}
	})
}
