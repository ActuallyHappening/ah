pub mod prelude {
	#![allow(unused_imports)]

	pub(crate) use crate::errors::*;
	pub use ah_tcita::*;
	pub(crate) use time::{Date, Duration, OffsetDateTime, Time, UtcDateTime};
}

pub mod errors {}

pub mod timetracker_sidbo {
	use ah_persistence::sidbo::SidboTcita;

	use crate::prelude::*;

	#[veciksi(lojban = "TODO span su'o le ckaji", glico = "Span properties")]
	pub struct SpanSidbo {
		pub(crate) id: SidboTcita,
		pub(crate) start: UtcDateTime,
		pub(crate) duration: Duration,
	}

	#[veciksi(
		lojban = "TODO billing company su'o le ckaji",
		glico = "Billing Company properties"
	)]
	pub struct BillingCompanySidbo {
		pub(crate) proper_name: String,
	}

	#[veciksi(lojban = "TODO project su'o le ckaji", glico = "Project properties")]
	pub struct ProjectSidbo {
		pub(crate) proper_name: String,
		pub(crate) billing_company: SidboTcita,
	}
}
