use ah_persistence::{SidboBuilder, packaji_sidbo, sidbo::SidboTcita};

use crate::{prelude::*, timetracker::Timetracker};

packaji_sidbo!(pub struct SpanFasnuSidbo {
	ckaji: FasnuCkaji,
	...
});

impl Ka_tcita for SpanFasnuSidbo {
	const TI_SELTCITA_BAU_LA_LOJBAN: &str = "TODO ah-timetracker fasnu span sidbo";
}

impl SpanFasnuSidbo {
	fn derive_name() -> String {
		format!(
			"{} noi randomness {}",
			SpanFasnuSidbo::TCITA,
			ah_lojban::quote(uuid::Uuid::new_v4().to_string()),
		)
	}
}

#[tcita("TODO ah-timetracker streaming fasnu ckaji")]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum FasnuCkaji {
	#[serde(rename = "TODO ah-timetracker start")]
	Start(Start),
	#[serde(rename = "TODO ah-timetracker stop")]
	Stop(Stop),
}

impl FasnuCkaji {
	pub fn time(&self) -> time::UtcDateTime {
		match self {
			FasnuCkaji::Start(start) => start.start.into(),
			FasnuCkaji::Stop(stop) => stop.stop.into(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Start {
	#[serde(with = "time::serde::iso8601")]
	start: time::OffsetDateTime,
	project: SidboTcita,
	billing_company: SidboTcita,
}

impl Start {
	pub fn start(&self) -> UtcDateTime {
		self.start.into()
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stop {
	#[serde(with = "time::serde::iso8601")]
	stop: time::OffsetDateTime,
	project: SidboTcita,
	billing_company: SidboTcita,
}

impl Stop {
	pub fn stop(&self) -> UtcDateTime {
		self.stop.into()
	}
}

#[derive(Debug)]
pub struct StartBuilder {
	pub start: time::UtcDateTime,
	pub project: SidboTcita,
	pub billing_company: SidboTcita,
}

#[derive(Debug)]
pub struct StopBuilder {
	pub stop: time::UtcDateTime,
	pub project: SidboTcita,
	pub billing_company: SidboTcita,
}

impl Timetracker {
	#[instrument(skip_all)]
	pub async fn start(&self, fasnu: StartBuilder) -> Result<SpanFasnuSidbo> {
		let sidbo =
			SidboBuilder::new(SpanFasnuSidbo::derive_name()).add_ckaji(FasnuCkaji::Start(Start {
				start: UtcDateTime::now().into(),
				project: fasnu.project,
				billing_company: fasnu.billing_company,
			}))?;
		let sidbo = self
			.persistence
			.add(sidbo)
			.await
			.wrap_err("Couldn't start")?;
		let ret = SpanFasnuSidbo::try_from(sidbo)?;
		info!("Started a span");
		Ok(ret)
	}

	#[instrument(skip_all)]
	pub async fn stop(&self, fasnu: StopBuilder) -> Result<SpanFasnuSidbo> {
		let sidbo =
			SidboBuilder::new(SpanFasnuSidbo::derive_name()).add_ckaji(FasnuCkaji::Stop(Stop {
				stop: fasnu.stop.into(),
				project: fasnu.project,
				billing_company: fasnu.billing_company,
			}))?;
		let sidbo = self
			.persistence
			.add(sidbo)
			.await
			.wrap_err("Couldn't stop")?;
		let ret = SpanFasnuSidbo::try_from(sidbo)?;
		info!("Stopped a span");
		Ok(ret)
	}
}

pub mod processing;
