use ah_persistence::{PersistenceEngine, PersistenceEngineBuilder, sidbo::SidboTcita};

use crate::{prelude::*, timetracker_ckaji::BillingCompanyCkaji};

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

	pub fn select_billing_company(&self, id: SidboTcita) -> Result<BillingCompanyCkaji> {
		todo!()
	}

	// pub fn get_billing_companies(&self) -> Result<BillingCompanyCkaji> {

	// }
}
