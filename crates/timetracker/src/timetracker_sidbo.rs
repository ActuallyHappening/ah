use ah_persistence::{
	packaji_sidbo,
	sidbo::{Sidbo, SidboTcita},
};
use serde::de::DeserializeOwned;

use crate::{
	prelude::*,
	timetracker_ckaji::{BillingCompanyCkaji, ProjectCkaji, TimetrackerCkaji},
};

#[derive(Debug, Deserialize)]
#[veciksi(
	lojban = "TODO ah-timetracker sidbo poi Caleb Yates",
	glico = "Timetracker sidbo for Caleb Yates"
)]
#[tcita("TODO ah-timetracker sidbo poi Caleb Yates")]
#[serde(try_from = "Sidbo")]
pub struct TimetrackerSidbo {
	id: SidboTcita,
	ckaji: TimetrackerCkaji,
	// created_at: Option<Infallible>,
	// lasted_mutated: Option<Infallible>,
}

impl TryFrom<Sidbo> for TimetrackerSidbo {
	type Error = Error;

	fn try_from(mut sidbo: Sidbo) -> Result<Self, Self::Error> {
		let timetracker = sidbo.extract_ckaji::<TimetrackerCkaji>()?;
		let id = sidbo.id();
		Ok(TimetrackerSidbo {
			id,
			ckaji: timetracker,
		})
	}
}

packaji_sidbo!(pub struct ProjectSidbo {
	ckaji: ProjectCkaji,
	...
});
// impl Ka_tcita for ProjectSidbo {
// 	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker ckaji project";
// }

packaji_sidbo!(pub struct BillingCompanySidbo {
	ckaji: BillingCompanyCkaji,
	...
});
// impl Ka_tcita for BillingCompanySidbo {
// 	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker ckaji billing company";
// }
