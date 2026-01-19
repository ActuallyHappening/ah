use ah_persistence::sidbo::{Sidbo, SidboTcita};

use crate::{prelude::*, timetracker_ckaji::TimetrackerCkaji};

#[derive(Debug)]
#[veciksi(
	lojban = "TODO ah-timetracker sidbo poi Caleb Yates",
	glico = "Timetracker sidbo for Caleb Yates"
)]
#[tcita("TODO ah-timetracker sidbo poi Caleb Yates")]
pub struct TimetrackerSidbo {
	id: SidboTcita,
	ckaji: TimetrackerCkaji,
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
