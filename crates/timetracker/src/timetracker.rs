use ah_persistence::{
	PersistenceEngine, PersistenceEngineBuilder, SidboBuilder, sidbo::SidboTcita,
};

use crate::{
	prelude::*, timetracker_ckaji::BillingCompanyCkaji, timetracker_sidbo::TimetrackerSidbo,
};

pub struct Timetracker {
	persistence: PersistenceEngine,
}

impl Timetracker {
	pub async fn new() -> Result<Self> {
		let persistence = PersistenceEngineBuilder::new()
			.wrap_err("Failed to create persistence engine")?
			.connect()
			.await?;
		Ok(Self { persistence })
	}

	pub async fn primary_sidbo(&self) -> Result<TimetrackerSidbo> {
		self
			.persistence
			.select(SidboTcita::from_tcita::<TimetrackerSidbo>())
			.await
			.wrap_err("Couldn't get primary sidbo")
	}

	pub async fn select_billing_company(&self, id: SidboTcita) -> Result<BillingCompanyCkaji> {
		self
			.persistence
			.select(SidboTcita::from_tcita::<BillingCompanyCkaji>())
			.await
			.wrap_err("Couldn't get billing company")
	}

	// pub fn get_billing_companies(&self) -> Result<HashSet<BillingCompanyCkaji>> {

	// }
}

pub struct AddBillingCompany {
	pub proper_name: String,
	pub short_name: String,
}

impl AddBillingCompany {
	pub fn tcita(&self) -> String {
		format!(
			"TODO ah-timetracker {} noi aka {}",
			ah_lojban::quote(self.proper_name.as_str()),
			ah_lojban::quote(self.short_name.as_str())
		)
	}
}

impl Timetracker {
	pub async fn add_billing_company(
		&self,
		company: AddBillingCompany,
	) -> Result<BillingCompanyCkaji> {
		// TODO: check that no billing companies with identical proper_name or short_name's exist

		let sidbo = SidboBuilder::new(company.tcita()).add_ckaji(BillingCompanyCkaji {
			proper_name: company.proper_name,
			short_name: company.short_name,
		})?;
		let mut sidbo = self.persistence.add(sidbo).await?;
		sidbo.extract_ckaji().wrap_err("Ckaji didn't save?")
	}
}
