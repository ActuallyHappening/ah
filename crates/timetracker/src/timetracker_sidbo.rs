use ah_persistence::sidbo::{Sidbo, SidboTcita};

use crate::{prelude::*, timetracker_ckaji::TimetrackerCkaji};

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
