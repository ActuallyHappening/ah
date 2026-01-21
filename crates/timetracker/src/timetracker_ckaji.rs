use ah_persistence::sidbo::SidboTcita;

use crate::prelude::*;

/// State about the timetracker itself
#[derive(Debug, Deserialize, Serialize)]
#[veciksi(
	lojban = "TODO ah-timetracker su'u internal state",
	glico = "Timetracker internal state abstraction"
)]
#[tcita("TODO ah-timetracker su'u internal state")]
pub struct TimetrackerCkaji {
	pub(crate) active: Option<Active>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Active {
	start: UtcDateTime,
}

#[veciksi(
	lojban = "TODO ah-timetracker su'u billing company",
	glico = "Billing Company abstraction"
)]
#[tcita("TODO ah-timetracker su'u billing company")]
#[derive(Debug, Serialize, Deserialize)]
pub struct BillingCompanyCkaji {
	pub(crate) proper_name: String,
	pub(crate) short_name: String,
}

impl BillingCompanyCkaji {
	pub fn match_short_name(&self, short_name: &str) -> bool {
		self.short_name == short_name
	}
}

#[veciksi(
	lojban = "TODO ah-timetracker su'u project",
	glico = "Project abstraction"
)]
#[tcita("TODO ah-timetracker su'u project")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCkaji {
	pub(crate) billing_company: SidboTcita,
	pub(crate) proper_name: String,
	pub(crate) short_name: String,
}

impl ProjectCkaji {
	pub fn match_short_name(&self, short_name: &str) -> bool {
		self.short_name == short_name
	}
}
