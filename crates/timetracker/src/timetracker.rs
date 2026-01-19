use ah_persistence::{PersistenceEngine, sidbo::SidboTcita};

use crate::{prelude::*, timetracker_sidbo::BillingCompanyCkaji};

pub struct Timetracker {
	persistence: PersistenceEngine,
}

impl Timetracker {
	pub fn select_billing_company(&self, id: SidboTcita) -> Result<BillingCompanyCkaji> {
		todo!()
	}
	
	// pub fn get_billing_companies(&self) -> Result<BillingCompanyCkaji> {

	// }
}
