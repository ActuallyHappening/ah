use surrealdb::types::{RecordId, SurrealValue};

#[derive(SurrealValue)]
pub struct BillableCompany {
	pub proper_name: String,
	pub short_name: String,
	pub created_at: surrealdb::types::Datetime,
}

impl BillableCompany {
	pub fn created_at(&self) -> time::UtcDateTime {
		self.created_at.into_inner().into()
	}
}

#[derive(SurrealValue)]
pub struct Project {
	pub billing_company: RecordId,
	pub proper_name: String,
	pub short_name: String,
	pub created_at: surrealdb::types::Datetime,
}

impl Project {
	pub fn created_at(&self) -> time::UtcDateTime {
		self.created_at().into_inner().into()
	}
}

#[derive(SurrealValue)]
pub struct Span {
	pub r#type: SpanType,
	pub time: surrealdb::types::Datetime,
	pub project: RecordId,
}

impl Span {
	pub fn time(&self) -> time::UtcDateTime {
		self.time.into_inner().into()
	}
}

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
			Self::Start => "START".into(),
			Self::Stop => "STOP".into(),
		}
	}
	fn from_value(value: surrealdb::types::Value) -> Result<Self, surrealdb::Error>
	where
		Self: Sized,
	{
		let str: String = value.try_into()?;
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
