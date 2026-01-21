use ah_persistence::{PersistenceEngine, SidboBuilder, packaji_sidbo, sidbo::SidboTcita};

use crate::{prelude::*, timetracker::Timetracker};

packaji_sidbo!(pub struct SpanFasnu {
	ckaji: FasnuCkaji,
	...
});

impl Ka_tcita for SpanFasnu {
	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker fasnu span sidbo";
}

impl SpanFasnu {
	fn derive_name() -> String {
		format!(
			"{} noi randomness {}",
			SpanFasnu::TCITA,
			ah_lojban::quote(uuid::Uuid::new_v4().to_string()),
		)
	}
}

#[tcita("TODO ah-timetracker streaming fasnu ckaji")]
#[derive(Deserialize, Serialize, Debug)]
pub enum FasnuCkaji {
	#[serde(rename = "TODO ah-timetracker start")]
	Start(Start),
	#[serde(rename = "TODO ah-timetracker stop")]
	Stop(Stop),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Start {
	#[serde(with = "time::serde::iso8601")]
	start: time::OffsetDateTime,
	project: SidboTcita,
	billing_company: SidboTcita,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stop {
	project: SidboTcita,
	billing_company: SidboTcita,
}

#[derive(Debug)]
pub struct StartBuilder {
	pub start: time::UtcDateTime,
	pub project: SidboTcita,
	pub billing_company: SidboTcita,
}

#[derive(Debug)]
pub struct StopBuilder {
	pub project: SidboTcita,
	pub billing_company: SidboTcita,
}

impl Timetracker {
	#[instrument(skip_all)]
	pub async fn start(&self, fasnu: StartBuilder) -> Result<SpanFasnu> {
		let sidbo =
			SidboBuilder::new(SpanFasnu::derive_name()).add_ckaji(FasnuCkaji::Start(Start {
				start: UtcDateTime::now().into(),
				project: fasnu.project,
				billing_company: fasnu.billing_company,
			}))?;
		let sidbo = self
			.persistence
			.add(sidbo)
			.await
			.wrap_err("Couldn't start")?;
		SpanFasnu::try_from(sidbo)
	}

	#[instrument(skip_all)]
	pub async fn stop(&self, fasnu: StopBuilder) -> Result<SpanFasnu> {
		let sidbo = SidboBuilder::new(SpanFasnu::derive_name()).add_ckaji(FasnuCkaji::Stop(Stop {
			project: fasnu.project,
			billing_company: fasnu.billing_company,
		}))?;
		let sidbo = self
			.persistence
			.add(sidbo)
			.await
			.wrap_err("Couldn't stop")?;
		SpanFasnu::try_from(sidbo)
	}
}
