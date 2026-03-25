use surrealdb::types::{RecordId, SurrealValue, Value};

pub mod actions;

#[derive(SurrealValue, Clone)]
pub struct BillableCompany {
	pub proper_name: String,
	pub short_name: String,
	pub created_at: surrealdb::types::Datetime,
	pub id: RecordId
}

impl BillableCompany {
	pub fn created_at(&self) -> time::UtcDateTime {
		time::UtcDateTime::from_unix_timestamp(self.created_at.timestamp()).unwrap()
	}
}

#[derive(SurrealValue, Clone)]
pub struct Project {
	pub billing_company: RecordId,
	pub proper_name: String,
	pub short_name: String,
	pub created_at: surrealdb::types::Datetime,
	pub id: RecordId
}

impl Project {
	pub fn created_at(&self) -> time::UtcDateTime {
		time::UtcDateTime::from_unix_timestamp(self.created_at.timestamp()).unwrap()
	}
}

#[derive(SurrealValue, Clone)]
pub struct Span {
	pub r#type: SpanType,
	pub time: surrealdb::types::Datetime,
	pub project: RecordId,
	pub id: RecordId
}

impl Span {
	pub fn time(&self) -> time::UtcDateTime {
		time::UtcDateTime::from_unix_timestamp(self.time.timestamp()).unwrap()
	}
}

#[derive(Clone)]
pub enum SpanType {
	Start,
	Stop,
}

impl SurrealValue for SpanType {
	fn kind_of() -> surrealdb::types::Kind {
		surrealdb::types::Kind::String
	}
	fn into_value(self) -> surrealdb::types::Value {
		match self {
			Self::Start => surrealdb::types::Value::String("START".into()),
			Self::Stop => surrealdb::types::Value::String("STOP".into()),
		}
	}
	fn from_value(value: surrealdb::types::Value) -> Result<Self, surrealdb::Error>
	where
		Self: Sized,
	{
		let Value::String(str) = value else {
			return Err(surrealdb::Error::thrown(format!("Unknown data type encountered when deserializing SpanType")))
		};
		Ok(match str.as_str() {
			"START" => Self::Start,
			"STOP" => Self::Stop,
			str => {
				return Err(surrealdb::Error::thrown(format!(
					"Unknown SpanType encountered when deserializing: '{str:?}'"
				)));
			}
		})
	}
}
