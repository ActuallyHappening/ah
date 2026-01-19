use ah_persistence::sidbo::{Sidbo, SidboTcita};
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

#[derive(Debug)]
pub struct PaCkajiSidbo<Ckaji> {
	id: SidboTcita,
	ckaji: Ckaji,
}

impl<Ckaji> TryFrom<Sidbo> for PaCkajiSidbo<Ckaji>
where
	Ckaji: DeserializeOwned + Ka_tcita,
{
	type Error = Error;

	fn try_from(mut sidbo: Sidbo) -> Result<Self, Self::Error> {
		let ckaji = sidbo.extract_ckaji::<Ckaji>()?;
		let id = sidbo.id();
		Ok(Self { id, ckaji })
	}
}

pub type ProjectSidbo = PaCkajiSidbo<ProjectCkaji>;
impl Ka_tcita for ProjectSidbo {
	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker ckaji project";
}

pub type BillingCompanySidbo = PaCkajiSidbo<BillingCompanyCkaji>;
impl Ka_tcita for BillingCompanySidbo {
	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker ckaji billing company";
}
